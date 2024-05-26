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

echo "Successful installation."
