pub(crate) const FISH_RECIPE_COMPLETIONS: &str = r#"function __fish_just_complete_recipes
    just --list 2> /dev/null | sed -e '1d; s/^\s*\([^[:space:]]*\)[^#]*$/\1/' -e 's/^\s*\([^[:space:]]*\)[^#]*# \(.*\)$/\1\t\2/'
end

# don't suggest files right off
complete -c just -n "__fish_is_first_arg" --no-files

# complete recipes
complete -c just -a '(__fish_just_complete_recipes)'

# autogenerated completions
"#;

pub(crate) const ZSH_COMPLETION_REPLACEMENTS: &[(&str, &str)] = &[
  (
    r#"    _arguments "${_arguments_options[@]}" \"#,
    r#"    local common=("#,
  ),
  (
    r#"'*--set=[Override <VARIABLE> with <VALUE>]' \"#,
    r#"'*--set[Override <VARIABLE> with <VALUE>]: :_just_variables' \"#,
  ),
  (
    r#"'-s+[Show information about <RECIPE>]' \
'--show=[Show information about <RECIPE>]' \"#,
    r#"'-s+[Show information about <RECIPE>]: :_just_commands' \
'--show=[Show information about <RECIPE>]: :_just_commands' \"#,
  ),
  (
    "'::ARGUMENTS -- Overrides and recipe(s) to run, defaulting to the first recipe in the \
     justfile:_files' \\
&& ret=0
\x20\x20\x20\x20
",
    r#")

    _arguments "${_arguments_options[@]}" $common \
        '1: :_just_commands' \
        '*: :->args' \
        && ret=0

    case $state in
        args)
            curcontext="${curcontext%:*}-${words[2]}:"

            local lastarg=${words[${#words}]}
            local recipe

            local cmds; cmds=(
                ${(s: :)$(_call_program commands just --summary)}
            )

            # Find first recipe name
            for ((i = 2; i < $#words; i++ )) do
                if [[ ${cmds[(I)${words[i]}]} -gt 0 ]]; then
                    recipe=${words[i]}
                    break
                fi
            done

            if [[ $lastarg = */* ]]; then
                # Arguments contain slash would be recognised as a file
                _arguments -s -S $common '*:: :_files'
            elif [[ $lastarg = *=* ]]; then
                # Arguments contain equal would be recognised as a variable
                _message "value"
            elif [[ $recipe ]]; then
                # Show usage message
                _message "`just --show $recipe`"
                # Or complete with other commands
                #_arguments -s -S $common '*:: :_just_commands'
            else
                _arguments -s -S $common '*:: :_just_commands'
            fi
        ;;
    esac

    return ret
"#,
  ),
  (
    "    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )",
    r#"    [[ $PREFIX = -* ]] && return 1
    integer ret=1
    local variables; variables=(
        ${(s: :)$(_call_program commands just --variables)}
    )
    local commands; commands=(
        ${${${(M)"${(f)$(_call_program commands just --list)}":#    *}/ ##/}/ ##/:Args: }
    )
"#,
  ),
  (
    r#"    _describe -t commands 'just commands' commands "$@""#,
    r#"    if compset -P '*='; then
        case "${${words[-1]%=*}#*=}" in
            *) _message 'value' && ret=0 ;;
        esac
    else
        _describe -t variables 'variables' variables -qS "=" && ret=0
        _describe -t commands 'just commands' commands "$@"
    fi
"#,
  ),
  (
    r#"_just "$@""#,
    r#"(( $+functions[_just_variables] )) ||
_just_variables() {
    [[ $PREFIX = -* ]] && return 1
    integer ret=1
    local variables; variables=(
        ${(s: :)$(_call_program commands just --variables)}
    )

    if compset -P '*='; then
        case "${${words[-1]%=*}#*=}" in
            *) _message 'value' && ret=0 ;;
        esac
    else
        _describe -t variables 'variables' variables && ret=0
    fi

    return ret
}

_just "$@""#,
  ),
];

pub(crate) const POWERSHELL_COMPLETION_REPLACEMENTS: &[(&str, &str)] = &[(
  r#"$completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText"#,
  r#"function Get-JustFileRecipes([string[]]$CommandElements) {
        $justFileIndex = $commandElements.IndexOf("--justfile");

        if ($justFileIndex -ne -1 && $justFileIndex + 1 -le $commandElements.Length) {
            $justFileLocation = $commandElements[$justFileIndex + 1]
        }

        $justArgs = @("--summary")

        if (Test-Path $justFileLocation) {
            $justArgs += @("--justfile", $justFileLocation)
        }

        $recipes = $(just @justArgs) -split ' '
        return $recipes | ForEach-Object { [CompletionResult]::new($_) }
    }

    $elementValues = $commandElements | Select-Object -ExpandProperty Value
    $recipes = Get-JustFileRecipes -CommandElements $elementValues
    $completions += $recipes
    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText"#,
)];

pub(crate) const BASH_COMPLETION_REPLACEMENTS: &[(&str, &str)] = &[
  (
    r#"            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi"#,
    r#"                if [[ ${cur} == -* ]] ; then
                    COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                    return 0
                elif [[ ${COMP_CWORD} -eq 1 ]]; then
                    local recipes=$(just --summary 2> /dev/null)

                    if echo "${cur}" | grep -qF '/'; then
                        local path_prefix=$(echo "${cur}" | sed 's/[/][^/]*$/\//')
                        local recipes=$(just --summary 2> /dev/null -- "${path_prefix}")
                        local recipes=$(printf "${path_prefix}%s\t" $recipes)
                    fi

                    if [[ $? -eq 0 ]]; then
                        COMPREPLY=( $(compgen -W "${recipes}" -- "${cur}") )
                        return 0
                    fi
                fi"#,
  ),
  (r#"            just)"#, r#"            "$1")"#),
];
