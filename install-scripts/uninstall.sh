#!/bin/bash

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

echo "Uninstallation completed successfully!"
