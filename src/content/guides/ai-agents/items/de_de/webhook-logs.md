Jeder Agent-Webhook hat ein eigenes Zustellungsprotokoll. Erreichbar von der [Webhook-Listen-Seite](https://fastcomments.com/auth/my-account/ai-agents/webhooks) über die **Protokolle**-Schaltfläche in jeder Webhook-Zeile.

### Was auf der Seite zu finden ist

Eine paginierte Tabelle mit einer Zeile pro Zustellversuch:

| Spalte | Bedeutung |
|---|---|
| Wann | Wann der Versuch stattgefunden hat. |
| Ereignis | Der Ereignistyp (`trigger.succeeded`, `approval.requested`, etc.). |
| Status | Der Zustellungsstatus. |
| StatusCode | HTTP-Statuscode, den Ihr Endpoint zurückgegeben hat, sofern verfügbar. |
| Beschreibung | Eine kurze Beschreibung des Ergebnisses. Bei fehlgeschlagenen Zustellungen, bei denen keine HTTP-Antwort empfangen wurde, wird die zugrundeliegende Fehlermeldung als `{error: <message>}` gespeichert. |

Die Seite unterstützt nur Paginierung – es gibt keine Filter nach Status, Ereignistyp oder Datumsbereich.

### Was Sie in den Protokollen tun können

- **Entscheiden Sie, ob ein Statuscode in 'Nicht erneut versuchen' stehen sollte.** Wenn Ihr Endpoint wiederholt denselben `4xx` zurückgibt, ist das ein starkes Indiz dafür, dass Sie ihn zu den **'Nicht erneut versuchen'-Statuscodes** hinzufügen sollten, damit die Plattform nicht weiter versucht.

### Fehlerinformationen

Wenn eine Zustellung ohne HTTP-Antwort fehlschlägt (DNS-Fehler, Verbindung abgelehnt, Timeout, TLS-Fehler etc.), wird die rohe Fehlermeldung als `{error: <message>}` aufgezeichnet. Die Plattform kategorisiert diese nicht in benannte Buckets wie `TIMEOUT` oder `DNS_ERROR` – lesen Sie die Fehlermeldung direkt, um zu sehen, was passiert ist.

Bei HTTP-Antworten zeigt die Spalte StatusCode den von Ihrem Endpoint zurückgegebenen Code. Häufige Fälle:

- **Bei allen Versuchen: `401` oder `403`** - Ihr Endpoint lehnt die Signatur ab. Überprüfen Sie, dass Sie das HMAC über `${timestamp}.${body}` berechnen und das richtige Secret verwenden. Siehe [Webhook-Signierung](#webhook-signing).
- **Bei allen Versuchen: `422`** - Ihr Endpoint hält die Nutzlast für ungültig. Entweder beheben Sie Ihren Endpoint, oder fügen Sie `422` zu den **'Nicht erneut versuchen'-Statuscodes** hinzu, wenn die Ablehnung für bestimmte Ereignisse erwartet wird.

### Kontext pro Zustellung

Jeder Protokolleintrag enthält:

- `webhookId` - welche Webhook-Konfiguration diese Zustellung erzeugt hat.
- `agentId` - um welchen Agenten es bei der Zustellung geht.
- `triggerId` or `approvalId` - der zugrundeliegende Datensatz.
- `domain` - die übereinstimmende Domain.

Sie können diese verwenden, um eine fehlgeschlagene Zustellung mit dem Lauf zu korrelieren, auf den sie sich im [Run History](#run-history) bezieht.

### Aufbewahrung

`AgentSyncLog`-Einträge haben eine einheitliche TTL von einem Jahr auf `createdAt`, unabhängig vom Ergebnis – erfolgreiche und fehlgeschlagene Zustellungen werden gleich lange aufbewahrt. Nach Ablauf der Aufbewahrungsfrist ist der Protokolleintrag verschwunden.

Wenn Sie eine langfristige Prüfung benötigen, ist das nachhaltige Muster: Lassen Sie den **Endpoint selbst** die empfangenen Zustellungen persistieren. Das entkoppelt Ihr Prüfprotokoll von der Aufbewahrungsrichtlinie der Plattform.

### Test senden

Die **Test senden**-Schaltfläche im Webhook-Konfigurationsformular schreibt eine gefälschte Zustellung in dieselbe Protokolltabelle, damit Sie die End-to-End-Konnektivität überprüfen können, bevor Sie sich auf echte Ereignisse verlassen. Testzustellungen sind im Protokoll eindeutig gekennzeichnet, sodass sie die Fehlerstatistiken der Produktion nicht verfälschen.

### Siehe auch

- [Webhooks-Übersicht](#webhooks-overview).
- [Webhook-Wiederholungen](#webhook-retries) für die Retry-Semantik, die diese Protokolle steuert.
- [Webhook-Signierung](#webhook-signing) dafür, wie Sie auf Ihrem Endpoint verifizieren können.