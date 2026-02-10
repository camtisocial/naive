pub fn init_script(shell: &str) -> Result<String, String> {
    let exe = std::env::current_exe()
        .map_err(|e| format!("failed to resolve naive binary path: {e}"))?
        .to_string_lossy()
        .into_owned();

    match shell {
        "bash" => Ok(bash_script(&exe)),
        "zsh" => Ok(zsh_script(&exe)),
        "fish" => Ok(fish_script(&exe)),
        _ => Err(format!("unsupported shell: {shell}")),
    }
}

fn bash_script(exe: &str) -> String {
    format!(
        r#"
__naive_widget() {{
    local result
    result="$({exe})"
    if [[ -n "$result" ]]; then
        READLINE_LINE="$result"
        READLINE_POINT=${{#result}}
    fi
}}
bind -x '"\C-b": __naive_widget'
"#
    )
}

fn zsh_script(exe: &str) -> String {
    format!(
        r#"
__naive_widget() {{
    local result
    result="$({exe})"
    if [[ -n "$result" ]]; then
        BUFFER="$result"
        CURSOR=${{#result}}
    fi
    zle redisplay
}}
zle -N __naive_widget
bindkey '^B' __naive_widget
"#
    )
}

fn fish_script(exe: &str) -> String {
    format!(
        r#"
function __naive_widget
    set -l result ({exe})
    if test -n "$result"
        commandline -r $result
        commandline -C (string length $result)
    end
end
bind \cb __naive_widget
"#
    )
}
