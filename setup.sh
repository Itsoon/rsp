cargo build --release
if [ $? -ne 0 ]; then
	echo "Compilation failed."
	exit 1
fi

sudo cp ./target/release/rsp /usr/local/bin/
if [ $? -ne 0 ]; then
	echo "Binary copy failed."
	exit 1
fi

read -p "Do you want to copy the configuration files? (y/N) " -n 1 -r
echo

if [[ $REPLY =~ ^[Yy]$ ]]; then
	# Backup existing configuration if it exists
	if [ -d "$HOME/.config/rsp" ]; then
		mv "$HOME/.config/rsp" "$HOME/.config/rsp_backup"
		if [ $? -ne 0 ]; then
			echo "Failed to rename existing 'rsp' directory."
			exit 1
		fi
	fi

	cp -r ./config/rsp/ "$HOME/.config/"
	if [ $? -ne 0 ]; then
		echo "Failed to copy configuration files."
		exit 1
	fi
fi

read -p "Do you want to copy the rofi script? (y/N) " -n 1 -r
echo

if [[ $REPLY =~ ^[Yy]$ ]]; then
	cp ./rsp_rofi.sh "$HOME/.config/rsp/"
	if [ $? -ne 0 ]; then
		echo "Failed to copy rofi script."
		exit 1
	fi
fi

echo "Successful installation."
