#!/bin/bash

echo ""
echo "    _                                            ___ "
echo "   /_\  _ _ ___   _  _ ___ _  _   ____  _ _ _ __|__ \ "
echo "  / _ \| '_/ -_) | || / _ \ || | (_-< || | '_/ -_)/_/ "
echo " /_/ \_\_| \___|  \_, \___/\_,_| /__/\_,_|_| \___(_) "
echo "                  |__/"
echo ""

# Get user input
read -r -p "Do you want to uninstall? (Y/n): " choice

# Check the user's choice
if [[ "$choice" == "Y" || "$choice" == "y" ]]; then

    # Set the target directory
    target_directory="/etc/squidvm"

    # Set /usr/bin
    bin="/usr/bin"

    # Remove the SquidVM file
    if [ -e "$target_directory/squidvm" ]; then
        rm "$target_directory/squidvm"
    fi

    # Remove the svdk file
    if [ -e "$target_directory/svdk" ]; then
        rm "$target_directory/svdk"
    fi

    # Remove the target directory if it is empty
    rmdir "$target_directory" 2>/dev/null

    # Remove the symbolic links in /usr/bin
    if [ -e "$bin/squidvm" ]; then
        rm -f "$bin/squidvm"
    fi
    if [ -e "$bin/svdk" ]; then
        rm -f "$bin/svdk"
    fi

    rm -- "$0"

    rm -rf "$target_directory"

    echo "Uninstallation completed successfully!"

else
    echo "Uninstallation canceled!"
fi
