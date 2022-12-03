del src\static\generated\*.* /q
node src/app
xcopy src\static\css src\static\generated\css\ /E /y
xcopy src\static\css src\static\generated\csv\ /E /y
xcopy src\static\images src\static\generated\images\ /E /y
xcopy src\static\js src\static\generated\js\ /E /y
