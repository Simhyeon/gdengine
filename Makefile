zip :
	if [ ! -f "./renderers/marp/bin/marp" ]; then
		echo "Marp doesn't exist renderers/marp/bin"
		exit 1
	fi
	if [ ! -f "./renderers/marp/bin/chrome" ]; then
		echo "Chrome doesn't exist in path renderers/marp/bin"
		exit 1
	fi
	if [ ! -f "./renderers/pandoc/bin/pandoc" ]; then
		echo "Pandoc doesn't exist in path renderers/pandoc/bin"
		exit 1
	fi
	cargo build --release
	strip ./target/release/gde
	cp ./target/release/gde ./gde
	tar -cvzf ./target/gdengine_x86_64-unknown-linux-gnu.tar.gz gde libs renderers
	rm gde
