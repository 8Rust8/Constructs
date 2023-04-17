$FTPServer = "127.0.0.1"
$FTPPort = "8080"

$tcpConnection = New-Object System.Net.Sockets.TcpClient($FTPServer, $FTPPort)
$tcpStream = $tcpConnection.GetStream()
$reader = New-Object System.IO.StreamReader($tcpStream)
$writer = New-Object System.IO.StreamWriter($tcpStream)
$writer.AutoFlush = $true

while ($tcpConnection.Connected) {
    #while ($tcpStream.DataAvailable -or $reader.Peek() -ne -1 ) {
    #    $reader.ReadLine()
    #}

    if ($tcpConnection.Connected) {
        Write-Host -NoNewline "prompt> "
        $command = Read-Host

        if ($command -eq "!*") {
            break
        }

        $writer.WriteLine($command) | Out-Null
        Start-Sleep -Milliseconds 10
    }
}

#$reader.Close()
#$writer.Close()
#$tcpConnection.Close()