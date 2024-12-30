# shellcheck shell=bash

# =============================================================================
#
# Utility functions for zoxide.
#

# pwd based on the value of _ZO_RESOLVE_SYMLINKS.
function __pj_pwd() {
    \builtin pwd -L
}

# cd + custom logic based on the value of _ZO_ECHO.
function __pj_cd() {
    # shellcheck disable=SC2164
    \builtin cd -- "$@"
}

# =============================================================================
#
# Hook configuration for zoxide.
#

# Hook to add new entries to the database.
# function __sp_hook() {
#     # shellcheck disable=SC2312
#     \command sp add -- "$(__sp_pwd)"
# }

# Initialize hook.
# shellcheck disable=SC2154
# if [[ ${precmd_functions[(Ie)__sp_hook]:-} -eq 0 ]] && [[ ${chpwd_functions[(Ie)__sp_hook]:-} -eq 0 ]]; then
#     chpwd_functions+=(__sp_hook)
# fi

# =============================================================================
#
# When using zoxide with --no-cmd, alias these internal functions as desired.
#

# Jump to a directory using interactive search.
# function __sp_find() {
#     \builtin local result
#     result="$(\command zoxide query --interactive -- "$@")" && __zoxide_cd "${result}"
# }

# Find

function __pj_compile() {
    command sp compile
}

function __pj_cd_root() {
    __sp_cd $(pk)
}

# Grep edit
function __pj_grep_edit() {
    RG_PREFIX="rg --column --line-number --no-heading --color=always --smart-case"
    INITIAL_QUERY="${*:-}"
    command fzf --ansi --disabled --query "$INITIAL_QUERY" --bind="ctrl-c:abort" --bind "start:reload:$RG_PREFIX {q} $(pk)" --bind "change:reload:sleep 0.1; $RG_PREFIX {q} || true" --delimiter : --preview 'bat --color=always {1} --highlight-line {2}' --preview-window 'up,60%,border-bottom,+{2}+3/3,~3' --bind 'enter:become($EDITOR {1} +{2})'
}

# STuff
function __pj_find_file() {
    \builtin local result
    result="$(\command fd . $(pk) -tf | fzf -- "$@")"

    if [[ -f ${result} ]]; then;
        $EDITOR "${result}"
    fi

    if [[ -d ${result} ]]; then;
        __pj_cd "${result}"
    fi
}

# function __sp_find_project_do() {
# if not currently in a project (give option to add current directory/project or to go to a project, in the later case this is ran)
# TODO show selected project on selection
function __pj_find_other_project() {
    # \builtin local result
    # result="$(\command fd . $(sp) | fzf -- "$@")" && $EDITOR "${result}"
    # initial
    # s="$(fd . $(sp) | fzf -- "$@")"
    # s="$(sp list | fzf -- "$@")"
    s="$(\command pk list | fzf --bind="ctrl-c:abort" -- "$@")"
    # secondary should show list of action
    # final should show menu of actions
    i=0
    # set -E
    while true
    do
        command -v $s
        d=$(echo "$s")
        if (( $i == 0 )); then
            if [[ $d == "" ]]; then
                break
            fi
            selection=$d
            menu=$(\command printf "find(edit)\nfind(show)\ngrep(edit)\ngo to project\ncompile\n" | fzf --bind="ctrl-c:abort")
            s=$menu "$@"
        elif (( $i == 1 )); then
            if [[ $d == "" ]]; then
                break
            fi
            action=$d
            if [[ $action == "find(edit)" ]]; then
                s="$(fd . -tf $(pk $selection) | fzf --bind="ctrl-c:abort" -- "$@")"
            elif [[ $action == "find(show)" ]]; then
                command fd . $(pk $selection)
                break
            elif [[ $action == "grep(edit)" ]]; then
                # TODO: This switches to a different directory on query wtf
                RG_PREFIX="rg --column --line-number --no-heading --color=always --smart-case"
                INITIAL_QUERY="${*:-}"
                __sp_cd ${selection}
                command fzf --ansi --disabled --query "$INITIAL_QUERY" --bind="ctrl-c:abort" --bind "start:reload:$RG_PREFIX {q} $(pk $selection)" --bind "change:reload:sleep 0.1; $RG_PREFIX {q} || true" --delimiter : --preview 'bat --color=always {1} --highlight-line {2}' --preview-window 'up,60%,border-bottom,+{2}+3/3,~3' --bind 'enter:become($EDITOR {1} +{2})'
                break
            elif [[ $action == "go to project" ]]; then
                __sp_cd ${selection}
                break
            elif [[ $action == "build project" ]]; then
                __sp_cd ${selection}
                break
            elif [[ $action == "run project" ]]; then
                __sp_cd ${selection}
                break
            elif [[ $action == "install project" ]]; then
                __sp_cd ${selection}
                break
            elif [[ $action == "vc" ]]; then
                __sp_cd ${selection}
                break
            else
                break
            fi
        elif (( $i == 2 )); then
            if [[ $d == "" ]]; then
                break
            fi
            __sp_cd ${selection}
            if [[ -f $d ]]; then;
                $EDITOR $d
            fi

            if [[ -d $d ]]; then;
                __sp_cd $d
            fi

            break
        else
            break
        fi
        ((i++));
    done
    # \builtin local result
    # result="$(\command sp list | fzf -- "$@")" && __sp_cd "${result}"
}

function pjc() {
    __sp_compile "$@"
}

function pjf() {
    __sp_find_file "$@"
}

function pjg() {
    __sp_grep_edit "$@"
}

# Go to project root
function pjr() {
    __sp_cd_root "$@"
}

function pjp() {
    __sp_find_other_project "$@"
}

# Completions.
# if [[ -o zle ]]; then
#     __parkour_result=''
#
#     function __parkour_z_complete() {
#         # Only show completions when the cursor is at the end of the line.
#         # shellcheck disable=SC2154
#         [[ "${#words[@]}" -eq "${CURRENT}" ]] || return 0
#
#         if [[ "${#words[@]}" -eq 2 ]]; then
#             # Show completions for local directories.
#             _cd -/
#
#         elif [[ "${words[-1]}" == '' ]]; then
#             # Show completions for Space-Tab.
#             # shellcheck disable=SC2086
#             __zoxide_result="$(\command zoxide query --exclude "$(__zoxide_pwd || \builtin true)" --interactive -- ${words[2,-1]})" || __zoxide_result=''
#
#             # Set a result to ensure completion doesn't re-run
#             compadd -Q ""
#
#             # Bind '\e[0n' to helper function.
#             \builtin bindkey '\e[0n' '__zoxide_z_complete_helper'
#             # Sends query device status code, which results in a '\e[0n' being sent to console input.
#             \builtin printf '\e[5n'
#
#             # Report that the completion was successful, so that we don't fall back
#             # to another completion function.
#             return 0
#         fi
#     }
#
#     function __zoxide_z_complete_helper() {
#         if [[ -n "${__zoxide_result}" ]]; then
#             # shellcheck disable=SC2034,SC2296
#             BUFFER="z ${(q-)__zoxide_result}"
#             __zoxide_result=''
#             \builtin zle reset-prompt
#             \builtin zle accept-line
#         else
#             \builtin zle reset-prompt
#         fi
#     }
#     \builtin zle -N __zoxide_z_complete_helper
#
#     [[ "${+functions[compdef]}" -ne 0 ]] && \compdef __zoxide_z_complete z
# fi
#
# =============================================================================
#
# To initialize zoxide, add this to your configuration (usually ~/.zshrc):
#
# eval "$(zoxide init zsh)"
