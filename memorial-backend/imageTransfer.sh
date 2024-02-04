newline=$'\n'
sqlVar="
INSERT INTO pictures (picture_id, filename, caption)
VALUES"

count=0
for imageFile in images/*
do
	count=$((count + 1))
	fileName=$(basename "$imageFile")
	caption=$(exiftool images/$fileName -S -ImageDescription)
	echo "$caption"
	caption=${caption:18}
	caption=$(echo "$caption" | sed "s/'/''/g")
	if [ $count -gt 1 ]
	then
		sqlVar="$sqlVar,"
	fi
	sqlVar="$sqlVar$newline($count, '$fileName', '$caption')"
done

sqlVar="$sqlVar ${newline}ON CONFLICT (picture_id) DO NOTHING;"
echo "$sqlVar" >> migrations/initialiseAll/up.sql
