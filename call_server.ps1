# Define the server address and port
$ServerAddress = "127.0.0.1"
$ServerPort = 4000

# Create a UDP client
$UdpClient = New-Object System.Net.Sockets.UdpClient

# Define the message to send
$Message = "This is a message to the udp server" + $ServerPort

$MessageBytes = [System.Text.Encoding]::UTF8.GetBytes($Message)

try {
    # Send the message to the server
    $UdpClient.Send($MessageBytes, $MessageBytes.Length, $ServerAddress, $ServerPort)
    Write-Host "Message sent to server: $Message"

    # Receive the response from the server
    $RemoteEndPoint = New-Object System.Net.IPEndPoint([System.Net.IPAddress]::Any, 0)
    $ResponseBytes = $UdpClient.Receive([ref]$RemoteEndPoint)
    $ResponseMessage = [System.Text.Encoding]::UTF8.GetString($ResponseBytes)
    Write-Host "Response received from server: $ResponseMessage"
} catch {
    Write-Error "An error occurred: $_"
} finally {
    # Close the UDP client
    $UdpClient.Close()
    Write-Host "UDP client closed."
}
