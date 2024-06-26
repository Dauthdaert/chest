use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Cmd {
    #[arg(value_enum)]
    shell: Shell,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Shell {
    Nu,
    Zsh,
}

impl Cmd {
    fn init_nu(&self) {
        let full = include_str!("../shell/chest.nu");
        println!("{full}");
    }

    fn init_zsh(&self) {
        let full = include_str!("../shell/chest.zsh");
        println!("{full}");
    }

    pub fn run(self) {
        match self.shell {
            Shell::Nu => self.init_nu(),
            Shell::Zsh => self.init_zsh(),
        }
    }
}
