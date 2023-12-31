#!/bin/bash

echo ""
echo "   _______      _______  _  __  "
echo "  / ____\ \    / /  __ \| |/ /  "
echo " | (___  \ \  / /| |  | | ' /   "
echo "  \___ \  \ \/ / | |  | |  <    "
echo "  ____) |  \  /  | |__| | . \   "
echo " |_____/    \/   |_____/|_|\_\  "
echo ""

# Get user input
read -r -p "Do you want to install the SquidVM Development Kit (SVDK)? (Y/n): " choice

# Check the user's choice
if [[ "$choice" == "Y" || "$choice" == "y" ]]; then

# Set the log file path
log_file="./svdk-install.log"
uninstall_script="https://github.com/Fragmenta-Company/SquidVM/raw/production/install-scripts/uninstall.sh"

# Function to log messages to the file
log_message() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') $1" >> "$log_file"
}

# Redirect stdout and stderr to the log file
exec > >(tee -a "$log_file") 2>&1

# Get the architecture of the Linux system
arch=$(uname -m)

# Map the architecture to the corresponding release file
case "$arch" in
    x86_64)
        file_name="squid-vm-x86_64-unknown-linux-gnu"
        svdk_file_name="svdk-x86_64-unknown-linux-gnu"
        ;;
    armv7l)
        file_name="squid-vm-armv7-unknown-linux-gnueabihf"
        svdk_file_name="svdk-armv7-unknown-linux-gnueabihf"
        ;;
    aarch64)
        file_name="squid-vm-aarch64-unknown-linux-gnu"
        svdk_file_name="svdk-aarch64-unknown-linux-gnu"
        ;;
    *)
        echo "Unsupported architecture: $arch"
        exit 1
        ;;
esac

# Set the target directory
target_directory="/etc/squidvm"

# Check if the target directory exists, if not, create it
if [ ! -d "$target_directory" ]; then
    mkdir -p "$target_directory"
fi

# Get the latest release URL from GitHub for SquidVM
release_url=$(curl -s https://api.github.com/repos/Fragmenta-Company/SquidVM/releases/latest | grep "browser_download_url.*$file_name" | cut -d '"' -f 4)

if [ -z "$release_url" ]; then
    echo "Error retrieving release URL for SquidVM. Please check the repository or try again later."
    exit 1
fi

# Download the SquidVM file
echo "Downloading $file_name..."
curl -L -s -o "$target_directory/$file_name" "$release_url"

# Get the latest release URL from GitHub for svdk
svdk_release_url=$(curl -s https://api.github.com/repos/Fragmenta-Company/SquidVM/releases/latest | grep "browser_download_url.*$svdk_file_name" | cut -d '"' -f 4)

if [ -z "$svdk_release_url" ]; then
    echo "Error retrieving release URL for svdk. Please check the repository or try again later."
    exit 1
fi

# Download the svdk file
echo "Downloading $svdk_file_name..."
curl -L -s -o "$target_directory/$svdk_file_name" "$svdk_release_url"

# Remove the existing SquidVM file if it exists
if [ -e "$target_directory/squidvm" ]; then
    rm "$target_directory/squidvm"
fi

# Remove the existing svdk file if it exists
if [ -e "$target_directory/svdk" ]; then
    rm "$target_directory/svdk"
fi

# Make the SquidVM file executable and rename it
chmod +x "$target_directory/$file_name"
mv "$target_directory/$file_name" "$target_directory/squidvm"

# Make the svdk file executable and rename it
chmod +x "$target_directory/$svdk_file_name"
mv "$target_directory/$svdk_file_name" "$target_directory/svdk"

# Download uninstall script
echo "Download uninstall script..."
curl -L -s -o "$target_directory/uninstall.sh" "$uninstall_script"

# Make it executable
chmod +x "$target_directory/uninstall.sh"

# Create a soft link in /usr/bin for SquidVM
ln -s -f "$target_directory/squidvm" "/usr/bin/squidvm"

# Create a soft link in /usr/bin for svdk
ln -s -f "$target_directory/svdk" "/usr/bin/svdk"

if [ "$0" != "$target_directory/update.sh" ]; then

    # Move the script to the target directory
    cp "$0" "$target_directory/update.sh"

    # Display a message indicating the move
    echo "Install script copied to $target_directory/update.sh for getting future updates"

fi

echo "Installation completed successfully!"
echo "The uninstall script is located in $target_directory!"

# Display the log file path
if [ -s "$log_file" ]; then
    echo "Check the log file for details: $log_file"
fi

else
  echo "Installation canceled!"
fi