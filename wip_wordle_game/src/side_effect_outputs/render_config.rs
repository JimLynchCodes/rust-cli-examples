use inquire::ui::{RenderConfig, Styled};

pub fn build_text_input_render_config() -> RenderConfig {
    let mut render_config = RenderConfig::default();
    render_config.prompt_prefix = Styled::new("").with_fg(inquire::ui::Color::LightRed);
    render_config.highlighted_option_prefix =
        Styled::new("➠").with_fg(inquire::ui::Color::LightYellow);
    render_config.selected_checkbox = Styled::new("☑").with_fg(inquire::ui::Color::LightGreen);
    render_config.scroll_up_prefix = Styled::new("⇞");
    render_config.scroll_down_prefix = Styled::new("⇟");
    render_config.unselected_checkbox = Styled::new("☐");
    render_config.answered_prompt_prefix = Styled::new("You guessed:");

    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new("❌").with_fg(inquire::ui::Color::LightRed));

    render_config.help_message = RenderConfig::default().prompt;

    render_config
}
