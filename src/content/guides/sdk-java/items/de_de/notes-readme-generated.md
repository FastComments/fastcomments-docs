### Broadcast-IDs

Du wirst sehen, dass du in einigen API-Aufrufen ein `broadcastId` übergeben sollst. Wenn du Ereignisse empfängst, erhältst du diese ID zurück, damit du das Ereignis ignorieren kannst, falls du Änderungen optimistisch auf dem Client
(was du vermutlich tun möchtest, da es die beste Benutzererfahrung bietet). Übergebe hier eine `UUID`. Die ID sollte so eindeutig sein, dass sie in einer Browsersitzung nicht zweimal vorkommt.