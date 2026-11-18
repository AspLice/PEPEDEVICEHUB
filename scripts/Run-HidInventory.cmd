@echo off
cd /d "%~dp0"
device-hub-scan.exe > hid-inventory.json
if errorlevel 1 (
  echo Scan failed. Send the console error message.
) else (
  echo Scan complete. Share hid-inventory.json in the chat.
)
pause
