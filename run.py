import time
from subprocess import call

# autorun script every 15 mins
# without openning window
while True:
    call(["pythonw","script.py"])
    time.sleep(900)

# to automate just create a bat file or whatever for the OS
# then cd to the directory and run python run.py or 
# start pythonw.exe run.py - to hide the command prompt window
# then exit, a template is at silentAutoRunWindows.bat
