### Broadcast-IDs

Sie werden feststellen, dass Sie in einigen API-Aufrufen ein `broadcastId` übergeben sollen. Wenn Sie Ereignisse empfangen, erhalten Sie diese ID zurück, sodass Sie das Ereignis ignorieren können, wenn Sie vorhaben, Änderungen auf dem Client optimistisch anzuwenden
(was Sie wahrscheinlich tun möchten, da es die beste Benutzererfahrung bietet). Übergeben Sie hier eine UUID. Die ID sollte ausreichend eindeutig sein, damit sie nicht zweimal in einer Browsersitzung auftritt.

### SSO (Einmalanmeldung)

Beispiele für SSO finden Sie unten.