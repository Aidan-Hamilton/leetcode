@REM I should so just have done this using vscode interface or powershell
@ECHO OFF
IF "%~1"=="" (
  GOTO test_all
) ELSE (
  GOTO test_id
)

:test_id
@cargo test test_%1
GOTO end

:test_all
@cargo t
GOTO end

:end