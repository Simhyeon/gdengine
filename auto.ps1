if (!(Test-Path "${PWD}/renderers/marp/bin/marp.exe")) {
	Write-Output "Marp doesn't exist renderers/marp/bin"
	exit 1
}

if (!(Test-Path "${PWD}/renderers/marp/bin/chrome/chrome.exe")) {
	Write-Output "Chrome doesn't exist in path renderers/marp/bin"
	exit 1
}

if (!(Test-Path "${PWD}/renderers/pandoc/bin/pandoc.exe")) {
	Write-Output "Pandoc doesn't exist in path renderers/pandoc/bin"
	exit 1
}

cargo build --release
Copy-Item ./target/release/gde.exe ./gde.exe
Compress-Archive -Path gde.exe,libs,renderers -DestinationPath gde.zip
Remove-Item gde.exe