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

echo "Successful installation."
