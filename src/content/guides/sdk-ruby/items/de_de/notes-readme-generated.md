### Broadcast-IDs

Sie werden sehen, dass Sie in einigen API-Aufrufen eine `broadcastId` übergeben müssen. Wenn Sie Ereignisse empfangen, erhalten Sie diese ID zurück, sodass Sie das Ereignis ignorieren können, wenn Sie planen, Änderungen optimistisch auf dem Client anzuwenden
(was Sie wahrscheinlich tun möchten, da dies das beste Erlebnis bietet). Übergeben Sie hier eine UUID. Die ID sollte eindeutig genug sein, um nicht zweimal in einer Browser-Sitzung aufzutreten.

### SSO (Single Sign-On)

SSO-Beispiele finden Sie unten.