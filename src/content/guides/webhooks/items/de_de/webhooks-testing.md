Im Webhooks-Admin gibt es `Send Test Payload`-Schaltflächen für jeden Ereignistyp (Create, Update, Delete). Die Create- und Update-Ereignisse senden ein Dummy WebhookComment-Objekt, während beim Testen von Delete ein Dummy-Request-Body mit nur einer ID gesendet wird.

Der Test macht zwei Aufrufe, um den Antwortcode für die "happy" (korrekter API-Schlüssel) und "sad" (ungültiger API-Schlüssel) Szenarien zu überprüfen.

Wenn der Test einen ungültigen API-Schlüssel sendet, sollten Sie einen Statuscode von 401 zurückgeben, damit der Test vollständig besteht. Wenn Sie den Wert des Tokens nicht korrekt prüfen, werden Sie einen Fehler sehen.

Dies dient dazu sicherzustellen, dass Sie die Anfrage korrekt authentifizieren.