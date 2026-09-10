## Troubleshooting

**"You do not have permission" when connecting.** Der angemeldete Benutzer ist kein API‑Administrator für das Konto.  
Bitten Sie den Kontoinhaber, die API‑Berechtigung auf der Seite Benutzer zu gewähren, oder verbinden Sie sich als Inhaber.

**The connection is labelled with the wrong site.** Die Zustimmungsseite verbindet das Konto, bei dem Sie zu diesem Zeitpunkt angemeldet waren.  
Trennen Sie die Verbindung in Zapier, wechseln Sie das Konto im FastComments‑Dashboard und verbinden Sie sich erneut.

**Events stopped arriving.** Überprüfen Sie die Seite Webhooks im Dashboard. Ein Abonnement, dessen Endpunkt sechs Tage lang fehlgeschlagen ist, wird automatisch deaktiviert und zeigt den Grund an. Aktivieren Sie es dort erneut, oder schalten Sie den Zap aus und wieder ein. Wenn das Abonnement vollständig fehlt, hat jemand es gelöscht; das Aus‑ und Einschalten des Zap erstellt es neu.

**Zapier says the account needs to be reconnected.** Die Verbindung wurde auf der Seite Verbundene Apps widerrufen, der Benutzer, der sie genehmigt hat, hat die API‑Berechtigung verloren, oder das Konto wurde gelöscht. Stellen Sie die Verbindung von Zapier aus erneut her.

**An action fails with "does not have write access".** Die Verbindung wurde mit nur Lesezugriff genehmigt. Stellen Sie die Verbindung erneut her und genehmigen Sie beide Berechtigungen.

**Rate limits and credits.** Aktionen und Suchen verbrauchen API‑Guthaben aus Ihrem Plan und unterliegen denselben Ratenbeschränkungen wie die REST‑API. Trigger verbrauchen kein Guthaben. Ein Zap, der an ein Limit stößt, wird von Zapier nach der von FastComments gemeldeten Verzögerung erneut versucht.

**The Domain dropdown is empty.** Domains werden angezeigt, sobald sie auf der Seite Domains im FastComments‑Dashboard konfiguriert sind. Lassen Sie das Feld leer, um Ereignisse für jede Domain zu erhalten.