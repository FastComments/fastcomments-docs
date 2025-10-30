del src\static\generated\*.* /q
REM echo Generating SDK documentation...
REM node src/sdk-guide-generator.js
if errorlevel 1 exit /b 1
node src/app
if errorlevel 1 exit /b 1
xcopy src\static\css src\static\generated\css\ /E /y
xcopy src\static\css src\static\generated\csv\ /E /y
xcopy src\static\images src\static\generated\images\ /E /y
xcopy src\static\js src\static\generated\js\ /E /y
