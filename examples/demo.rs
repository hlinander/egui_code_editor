#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{self, egui, CreationContext};
use egui_code_editor::{self, CodeEditor, ColorTheme, Syntax};

const THEMES: [ColorTheme; 8] = [
    ColorTheme::AYU,
    ColorTheme::AYU_MIRAGE,
    ColorTheme::AYU_DARK,
    ColorTheme::GITHUB_DARK,
    ColorTheme::GITHUB_LIGHT,
    ColorTheme::GRUVBOX,
    ColorTheme::GRUVBOX_LIGHT,
    ColorTheme::SONOKAI,
];

const SYNTAXES: [SyntaxDemo; 5] = [
    SyntaxDemo::new(
        "Assembly",
        r#"ADD R1,R1,R3 ; R1 = R1 + R3
ADD R1,R1, 3 ; R1 = R1 + 3
LD R6,NUMBER ; R6 = Mem(Address of Number)
BRz LOOP ; if previous zero go to address LOOP"#,
    ),
    SyntaxDemo::new(
        "Lua",
        r#"-- Binary Search
function binarySearch(list, value)
    local function search(low, high)
        if low > high then return false end
        local mid = math.floor((low+high)/2)
        if list[mid] > value then return search(low,mid-1) end
        if list[mid] < value then return search(mid+1,high) end
        return mid
    end
    return search(1,#list)
end"#,
    ),
    SyntaxDemo::new(
        "Rust",
        r#"// Code Editor
CodeEditor::default()
    .id_source("code editor")
    .with_rows(12)
    .with_fontsize(14.0)
    .with_theme(self.theme)
    .with_syntax(self.syntax.to_owned())
    .with_numlines(true)
    .vscroll(true)
    .show(ui, &mut self.code);"#,
    ),
    SyntaxDemo::new(
        "Shell",
        r#"#!/bin/bash
user=p4ymak
if grep $user /etc/passwd
then
echo "The user $user Exists"
fi"#,
    ),
    SyntaxDemo::new(
        "SQL",
        r#"select now(); -- what time it is?
WITH employee_ranking AS (
  SELECT 
    employee_id as real, 
    last_name, 
    first_name, 
    salary, 
    dept_id
    RANK() OVER (PARTITION BY dept_id ORDER BY salary DESC) as ranking
  FROM employee
)"#,
    ),
];

#[derive(Clone, Copy)]
struct SyntaxDemo {
    name: &'static str,
    example: &'static str,
}

impl SyntaxDemo {
    const fn new(name: &'static str, example: &'static str) -> Self {
        SyntaxDemo { name, example }
    }
    fn syntax(&self) -> Syntax {
        match self.name {
            "Assembly" => Syntax::asm(),
            "Lua" => Syntax::lua(),
            "Rust" => Syntax::rust(),
            "Shell" => Syntax::shell(),
            "SQL" => Syntax::sql(),
            _ => Syntax::shell(),
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(true)
            .with_transparent(false)
            .with_resizable(true)
            .with_maximized(false)
            .with_drag_and_drop(true)
            .with_inner_size([900.0, 280.0])
            .with_min_inner_size([280.0, 280.0]),

        ..Default::default()
    };

    eframe::run_native(
        "Egui Code Editor",
        options,
        Box::new(|cc| Box::new(CodeEditorDemo::new(cc))),
    )
}

#[derive(Default)]
struct CodeEditorDemo {
    code: String,
    theme: ColorTheme,
    syntax: Syntax,
    example: bool,
}
impl CodeEditorDemo {
    fn new(_cc: &CreationContext) -> Self {
        let rust = SYNTAXES[2];
        CodeEditorDemo {
            code: rust.example.to_string(),
            theme: ColorTheme::GRUVBOX,
            syntax: rust.syntax(),
            example: true,
        }
    }
}
impl eframe::App for CodeEditorDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("theme_picker").show(ctx, |ui| {
            ui.heading("Theme");
            egui::ScrollArea::both().show(ui, |ui| {
                for theme in THEMES.iter() {
                    if ui
                        .selectable_value(&mut self.theme, *theme, theme.name())
                        .clicked()
                    {
                        if theme.is_dark() {
                            ctx.set_visuals(egui::Visuals::dark());
                        } else {
                            ctx.set_visuals(egui::Visuals::light());
                        }
                    };
                }
            });
        });

        egui::SidePanel::right("syntax_picker").show(ctx, |ui| {
            ui.horizontal(|h| {
                h.heading("Syntax");
                h.checkbox(&mut self.example, "Example");
            });
            egui::ScrollArea::both().show(ui, |ui| {
                for syntax in SYNTAXES.iter() {
                    if ui
                        .selectable_label(self.syntax.language() == syntax.name, syntax.name)
                        .clicked()
                    {
                        self.syntax = syntax.syntax();
                        if self.example {
                            self.code = syntax.example.to_string()
                        }
                    };
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            CodeEditor::default()
                .id_source("code editor")
                .with_rows(16)
                .with_fontsize(14.0)
                .with_theme(self.theme)
                .with_syntax(self.syntax.to_owned())
                .with_numlines(true)
                .vscroll(true)
                .show(ui, &mut self.code);
        });
    }
}
