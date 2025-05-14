#####################################################################
#
# This entire file was copy + pasted from my personal .bashrc config
# at: https://github.com/nasccped/bashrc.conf
#
# It's serves just to style the bash prompt
#
#####################################################################

RESET_ESCAPE="\e[0m"
GREEN_ESCAPE="\e[1;92m"
YELLOW_ESCAPE="\e[1;93m"
BLUE_ESCAPE="\e[1;94m"
CYAN_ESCAPE="\e[1;96m"
WHITE_ESCAPE="\e[1;97m"

get_user_name() {
  echo -e "${GREEN_ESCAPE}$(whoami)${RESET_ESCAPE}"
}

get_cur_dir() {
  local curdir=$(pwd)
  if [ "$curdir" == "$HOME" ]; then
    curdir="~"
  else
    curdir=$(pwd | rev | cut -d'/' -f 1 | rev)
  fi
  echo -e "${BLUE_ESCAPE}$curdir${RESET_ESCAPE}"
}

get_branch() {
  local curbranch=$(git branch 2> /dev/null | grep '*' | sed 's/* /*/')
  if [ -z $curbranch ]; then
    curbranch="?"
  else
    curbranch="${CYAN_ESCAPE}$curbranch${RESET_ESCAPE}"
  fi
  local gitbranch="${YELLOW_ESCAPE}git:($curbranch${YELLOW_ESCAPE})${RESET_ESCAPE}"
  echo -e "$gitbranch"
}

get_ps1() {
  local luser=$(get_user_name)
  # local ldir=$(get_cur_dir) this won't work
  # local lbranch=$(get_branch) this won't work too
  echo -e "${WHITE_ESCAPE}[$luser${WHITE_ESCAPE}.\$(get_cur_dir) \$(get_branch)${WHITE_ESCAPE}]\$${RESET_ESCAPE}"
}

echo -e "Welcome to the ${YELLOW_ESCAPE}Kojamp${RESET_ESCAPE} container!"
echo -e "If you're new here, type \`${GREEN_ESCAPE}kojamp-docker${RESET_ESCAPE}\` or"
echo -e "check out the official repository (${CYAN_ESCAPE}https://github.com/nasccped/kojamp${RESET_ESCAPE})"

export PS1="$(get_ps1) "
