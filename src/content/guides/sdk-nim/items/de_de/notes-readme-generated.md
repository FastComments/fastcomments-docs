### Broadcast-IDs

Sie werden sehen, dass Sie in einigen API-Aufrufen ein `broadcastId` übergeben sollen. Wenn Sie Ereignisse erhalten, bekommen Sie diese ID zurück, sodass Sie das Ereignis ignorieren können, falls Sie Änderungen optimistisch auf dem Client anwenden möchten
(was Sie wahrscheinlich tun werden, da es die beste Nutzererfahrung bietet). Übergeben Sie hier eine UUID. Die ID sollte eindeutig genug sein, um in einer Browsersitzung nicht zweimal aufzutreten.

### SSO (Single Sign-On)

Für SSO-Beispiele siehe unten.