#!/usr/bin/env bash

# init.sh - Xqhare's Pantheon Project Initializer
# Follows EDDD principles to move from template to documented project.

set -e

# Colors for feedback
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}--- Pantheon Project Initializer ---${NC}"
echo #

# 1. Gather Information
read -p "Project/Deity Name (e.g., Hera): " NAME
read -p "historical-startup-notes file path (e.g., /home/xqhare/Programming/project_ideas/idea.md): " HSN
read -p "Deity Description (e.g., Etruscan Goddess of Women): " DEITY_DESC
read -p "Project Description (e.g., A system monitor): " DESC
echo #

if [ -z "$HSN" ]; then
    echo "Error: historical-startup-notes file path is required."
    exit 1
fi

if [ ! -f "$HSN" ]; then
    echo "Error: historical-startup-notes file does not exist."
    exit 1
fi

if [ -z "$NAME" ]; then
    echo "Error: Project Name is required."
    exit 1
fi

echo #
echo -e "${BLUE}Initializing $NAME...${NC}"
echo #
echo Replace Placeholders...
echo #

CURRENT_YEAR=$(date +%Y)
# We use a temp file to avoid sed issues across different environments
FILES=$(find . -type f -not -path '*/.*' -not -name 'init.sh' -not -path './target/*')

for FILE in $FILES; do
    # Replace $NAME
    sed -i "s/\$NAME/$NAME/g" "$FILE"
    # Replace [Deity Description]
    sed -i "s/\[Deity Description\]/$DEITY_DESC/g" "$FILE"
    # Replace [Description]
    sed -i "s/\[Description\]/$DESC/g" "$FILE"
    # Replace $YEAR
    sed -i "s/\$YEAR/$CURRENT_YEAR/g" "$FILE"
done

echo Specific fix for Cargo.toml and URLs
echo #
NAME_LOWER=$(echo "$NAME" | tr '[:upper:]' '[:lower:]')
# Fix Cargo.toml name
sed -i "s/name = \"$NAME\"/name = \"$NAME_LOWER\"/g" Cargo.toml
# Fix GitHub URLs (making them lowercase)
sed -i "s/github.com\/xqhare\/$NAME/github.com\/xqhare\/$NAME_LOWER/g" README.md
sed -i "s/github.com\/xqhare\/$NAME/github.com\/xqhare\/$NAME_LOWER/g" CONTRIBUTING.md

echo Move historical-startup-notes to root
mv "$HSN" "historical-startup-notes.md"
echo #
echo Remove initialization instruction from README
sed -i '/Run `bash init.sh`/d' README.md

echo #
echo Script self-destruct...
rm init.sh
echo #
echo Committing changes...
echo #
# Personal git add and commit all alias
git adcta "chore: run init.sh and setup project"
echo #
echo -e "${BLUE}Changes committed.${NC}"
echo #
echo -e "${BLUE}init.sh removed. Happy coding.${NC}"
echo #
echo -e "${GREEN}Success! $NAME is ready for development.${NC}"

exit 0
