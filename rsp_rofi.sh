output=$(rsp list)

if [ $? -ne 0 ]; then
	echo "Failed to execute 'rsp list'."
	exit 1
fi

profiles=$(echo "$output" | awk '{ $1=""; print $0 }' | sed 's/^ *//')

if [ -z "$profiles" ]; then
	echo "No profiles found."
	exit 1
fi

selected_profile=$(echo "$profiles" | rofi -dmenu -p "Select profile")

if [ -z "$selected_profile" ]; then
	echo "No profile selected."
	exit 1
fi

rsp "$selected_profile"

if [ $? -ne 0 ]; then
	echo "Failed to execute 'rsp $selected_profile'."
	exit 1
fi

echo "Profile '$selected_profile' executed successfully."
