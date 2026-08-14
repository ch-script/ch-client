use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};

pub fn setup_ch_theme() {
    let mut theme = RenderConfig::default();

    // prompts and prefixes
    theme.prompt = StyleSheet::new().with_fg(Color::LightCyan).with_attr(Attributes::BOLD);
    theme.prompt_prefix = Styled::new("ch prompt").with_fg(Color::LightMagenta);
    theme.answered_prompt_prefix = Styled::new("ch success").with_fg(Color::LightGreen);
    theme.help_message = StyleSheet::new().with_fg(Color::DarkGrey);

    // options selection
    theme.option = StyleSheet::new().with_fg(Color::White);
    theme.highlighted_option_prefix = Styled::new("❯ ").with_fg(Color::LightMagenta);

    inquire::set_global_render_config(theme);
}