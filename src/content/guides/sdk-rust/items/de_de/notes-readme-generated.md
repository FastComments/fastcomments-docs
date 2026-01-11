### Broadcast-IDs

Sie werden sehen, dass Sie in einigen API-Aufrufen eine `broadcastId` übergeben sollen. Wenn Sie Events erhalten, bekommen Sie diese ID zurück, sodass Sie das Event ignorieren können, falls Sie vorhaben, Änderungen auf dem Client optimistisch anzuwenden
(was Sie wahrscheinlich tun möchten, da es die beste Erfahrung bietet). Geben Sie hier eine UUID an. Die ID sollte ausreichend einzigartig sein, damit sie in einer Browsersitzung nicht zweimal vorkommt.