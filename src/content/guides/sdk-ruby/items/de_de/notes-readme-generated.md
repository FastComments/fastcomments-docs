### Broadcast-IDs

Sie werden sehen, dass Sie in einigen API-Aufrufen eine `broadcastId` übergeben sollen. Wenn Sie Ereignisse empfangen, erhalten Sie diese ID zurück, sodass Sie das Ereignis ignorieren können, wenn Sie planen, Änderungen optimistisch auf dem Client anzuwenden
(was Sie wahrscheinlich tun möchten, da es die beste Erfahrung bietet). Übergeben Sie hier eine UUID. Die ID sollte eindeutig genug sein, um in einer Browsersitzung nicht zweimal aufzutreten.

### SSO (Single Sign-On)

Für SSO-Beispiele siehe unten.