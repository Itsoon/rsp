git fetch

UPDATES=$(git log HEAD..origin/main --oneline)

if [ -z "$UPDATES" ]; then
	echo "No updates available."
else
	echo "Updates found. Pulling changes..."
	git pull
	if [ $? -ne 0 ]; then
		echo "Failed to pull the latest changes."
		exit 1
	fi

	echo "Running setup script..."
	./setup.sh
	if [ $? -ne 0 ]; then
		echo "Setup script failed."
		exit 1
	fi
fi

echo "Update process completed."
