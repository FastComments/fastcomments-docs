## Fehlerbehebung

**„Sie haben keine Berechtigung" beim Verbinden.** Der angemeldete Benutzer ist kein API-Administrator auf dem Konto.
Bitten Sie den Kontoinhaber, die API-Berechtigung auf der Benutzerseite zu erteilen, oder verbinden Sie sich als Kontoinhaber.

**Die Verbindung ist der falschen Website zugeordnet.** Die Zustimmungsseite verbindet das Konto, bei dem Sie zum
Zeitpunkt der Verbindung angemeldet waren. Trennen Sie die Verbindung in Zapier, wechseln Sie im FastComments-Dashboard das Konto, und verbinden Sie sich erneut.

**Ereignisse kommen nicht mehr an.** Prüfen Sie die Webhooks-Seite im Dashboard. Ein Abonnement, dessen Endpunkt
sechs Tage lang fehlgeschlagen ist, wird automatisch deaktiviert und zeigt den Grund an. Aktivieren Sie es dort wieder, oder schalten Sie den Zap aus und wieder ein. Wenn das Abonnement vollständig fehlt, wurde es von jemandem gelöscht; das Aus- und Wiedereinschalten des Zaps erstellt es neu.

**Zapier meldet, dass das Konto neu verbunden werden muss.** Die Verbindung wurde über die Seite „Connected Apps"
widerrufen, der Benutzer, der sie genehmigt hat, hat die API-Berechtigung verloren, oder das Konto wurde gelöscht. Verbinden Sie sich erneut über Zapier.

**Eine Aktion schlägt mit „does not have write access" fehl.** Die Verbindung wurde nur mit Leseberechtigung
genehmigt. Verbinden Sie sich erneut und genehmigen Sie beide Berechtigungen.

**Ratenlimits und Credits.** Aktionen und Suchvorgänge verbrauchen API-Credits aus Ihrem Plan und unterliegen denselben
Ratenlimits wie die REST-API. Trigger verbrauchen keine. Ein Zap, der auf ein Limit stößt, wird von Zapier
nach der von FastComments gemeldeten Verzögerung erneut ausgeführt.

**Die Domain-Dropdown-Liste ist leer.** Domains erscheinen, sobald sie auf der Domains-Seite im
FastComments-Dashboard konfiguriert sind. Lassen Sie das Feld leer, um Ereignisse für alle Domains zu erhalten.