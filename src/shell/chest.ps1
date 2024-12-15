# Source this in your PowerShell profile
# Invoke-Expression (& { (chest init powershell | Out-String) })

# Workaround for fact that PSReadLine seems to clear screen
# after keyboard shortcut action is executed, and to work around a UTF8
# PSReadLine issue (GitHub PSFZF issue #71)
function InvokePromptHack()
{
	$previousOutputEncoding = [Console]::OutputEncoding
	[Console]::OutputEncoding = [Text.Encoding]::UTF8

	try {
		[Microsoft.PowerShell.PSConsoleReadLine]::InvokePrompt()
	} finally {
		[Console]::OutputEncoding = $previousOutputEncoding
	}
}

function Invoke-Chest {
	param(
		[string]$Query
    )

	Begin {
		# prepare to start process:
        $process = New-Object System.Diagnostics.Process
        $process.StartInfo.FileName = "chest"
		$process.StartInfo.Arguments = "search -i $Query"
        $process.StartInfo.StandardOutputEncoding = [System.Text.Encoding]::UTF8
        $process.StartInfo.RedirectStandardInput = $true
        $process.StartInfo.RedirectStandardOutput = $true
		$process.StartInfo.UseShellExecute = $false

        # Adding event handlers for stdout:
    	$stdOutEventId = "ChestStdOutEh-" + [System.Guid]::NewGuid()
    	$stdOutEvent = Register-ObjectEvent -InputObject $process `
    		-EventName 'OutputDataReceived' `
			-SourceIdentifier $stdOutEventId

        $processHasExited = New-Object PSObject -property @{flag = $false}
        # register on exit:
        $scriptBlockExited = {
            $Event.MessageData.flag = $true
        }
        $exitedEventId = "ChestExitedEh-" + [System.Guid]::NewGuid()
        $exitedEvent = Register-ObjectEvent -InputObject $process `
        	-Action $scriptBlockExited -EventName 'Exited' `
			-SourceIdentifier $exitedEventId `
        	-MessageData $processHasExited

        $process.Start() | Out-Null
        $process.BeginOutputReadLine() | Out-Null

		$cleanup = [scriptblock] {
			try {
           		$process.StandardInput.Close() | Out-Null
				$process.WaitForExit()
			} catch {
				# do nothing
			}

			try {
				$stdOutEvent,$exitedEvent | ForEach-Object {
					Stop-Job $_  -ErrorAction SilentlyContinue
					Remove-Job $_ -Force  -ErrorAction SilentlyContinue
				}
			} catch {

			}

			# Events seem to be generated out of order - therefore, we need sort by time created.
			Get-Event -SourceIdentifier $stdOutEventId | `
				Sort-Object -Property TimeGenerated | `
				Where-Object { $null -ne $_.SourceEventArgs.Data } | ForEach-Object {
					Write-Output $_.SourceEventArgs.Data
					Remove-Event -EventIdentifier $_.EventIdentifier
				}
		}
	}

	End {
		& $cleanup
	}
}

function Invoke-ChestSearchReadlineHandler {
	$result = $null
	$line = $null
	$cursor = $null
	[Microsoft.PowerShell.PSConsoleReadline]::GetBufferState([ref]$line, [ref]$cursor)

	$result = Invoke-Chest -Query "$line"

	InvokePromptHack

	if (-not [string]::IsNullOrEmpty($result)) {
		[Microsoft.PowerShell.PSConsoleReadLine]::Replace(0, $line.Length, $result)
	}
}

function SetPsReadlineShortcut($Chord, $BriefDesc, $Desc, [scriptblock]$ScriptBlock)
{
	Set-PSReadlineKeyHandler -Key $Chord -Description $Desc -BriefDescription $BriefDesc -ScriptBlock $ScriptBlock
	if ($(Get-PSReadLineOption).EditMode -eq [Microsoft.PowerShell.EditMode]::Vi) {
		Set-PSReadlineKeyHandler -Key $Chord -ViMode Command -Description $Desc -BriefDescription $BriefDesc -ScriptBlock $ScriptBlock
	}
}

if (Get-Module -ListAvailable -Name PSReadline) {
	SetPsReadlineShortcut 'Ctrl+h' 'Chest search' 'Invoke Chest Search for commands' { Invoke-ChestSearchReadlineHandler }
} else {
	Write-Warning "PSReadline module not found - keyboard handlers not installed"
}
