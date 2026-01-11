### Broadcast-IDs

Sie werden sehen, dass Sie in einigen API-Aufrufen ein `broadcastId` übergeben sollen. Wenn Sie Ereignisse erhalten, bekommen Sie diese ID zurück, sodass Sie das Ereignis ignorieren können, falls Sie Änderungen auf dem Client optimistisch anwenden möchten (was Sie vermutlich tun sollten, da es die beste Nutzererfahrung bietet). Übergeben Sie hier eine UUID. Die ID sollte so eindeutig sein, dass sie innerhalb einer Sitzung nicht zweimal vorkommt.

```swift
let broadcastId = UUID().uuidString
```