use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::compiler::Bifurcator;
use crate::frontend::{Lexer, Parser, Program};
use crate::runtime::Runtime;

pub struct ModuleLoader {
    loaded_files: HashSet<PathBuf>,
}

impl ModuleLoader {
    pub fn new() -> Self {
        Self {
            loaded_files: HashSet::new(),
        }
    }

    pub fn load_program(&mut self, entry_path: &Path) -> Result<Program, String> {
        let canonical_entry = entry_path
            .canonicalize()
            .map_err(|e| format!("Could not resolve path '{}': {}", entry_path.display(), e))?;

        let mut combined_program = Program {
            imports: Vec::new(),
            structs: Vec::new(),
            functions: Vec::new(),
        };

        self.load_recursive(&canonical_entry, &mut combined_program)?;
        Ok(combined_program)
    }

    fn load_recursive(
        &mut self,
        canonical_path: &Path,
        accumulator: &mut Program,
    ) -> Result<(), String> {
        if self.loaded_files.contains(canonical_path) {
            return Ok(());
        }
        self.loaded_files.insert(canonical_path.to_path_buf());

        let source = fs::read_to_string(canonical_path)
            .map_err(|e| format!("Error reading file '{}': {}", canonical_path.display(), e))?;

        let mut lexer = Lexer::new(&source);
        let tokens = lexer
            .tokenize()
            .map_err(|e| format!("In {}: {}", canonical_path.display(), e))?;

        let mut parser = Parser::new(tokens);
        let mut program = parser
            .parse_program()
            .map_err(|e| format!("In {}: {}", canonical_path.display(), e))?;

        let parent_dir = canonical_path
            .parent()
            .unwrap_or_else(|| Path::new("."));

        // Process imports relative to current file's directory
        for import_stmt in &program.imports {
            let relative_import_path = &import_stmt.path;
            let mut resolved = parent_dir.join(relative_import_path);

            // Automatically add .el extension if omitted
            if resolved.extension().is_none() {
                resolved.set_extension("el");
            }

            let canonical_import = resolved.canonicalize().map_err(|e| {
                format!(
                    "Module Import Error at {}:{}:{}: Cannot find import '{}' ({})",
                    canonical_path.display(),
                    import_stmt.line,
                    import_stmt.col,
                    relative_import_path,
                    e
                )
            })?;

            self.load_recursive(&canonical_import, accumulator)?;
        }

        // Merge structs & functions into accumulator
        accumulator.structs.append(&mut program.structs);
        accumulator.functions.append(&mut program.functions);

        Ok(())
    }
}

pub fn execute_file(path: &Path) -> Result<(), String> {
    let mut loader = ModuleLoader::new();
    let program = loader.load_program(path)?;

    let mut bifurcator = Bifurcator::new();
    bifurcator.compile_program(program)?;

    let mut runtime = Runtime::new(bifurcator);
    runtime.execute("main")?;
    Ok(())
}
