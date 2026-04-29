Agent-Webhooks werden bei Fehlern erneut versucht. Die Zustellung ist **fire-and-forget aus Sicht des Agents** – eine fehlgeschlagene Zustellung blockiert nicht die Ausführung des Agents und rollt keine Aktionen zurück – und eine Warteschlange + Cron steuern asynchron die Wiederholungsversuche.

### Warteschlangenmodell

Jedes Ereignis wird **einmal pro übereinstimmendem Webhook** in die Warteschlange gestellt. Wenn Sie also drei Webhooks haben, die für ein bestimmtes Agent + Domain auf `trigger.succeeded` abonniert sind, stellt die Plattform drei Zustellungen in die Warteschlange; jede wird unabhängig zugestellt und erneut versucht. Ein Fehler bei einem Webhook beeinflusst die anderen niemals.

### Was wird erneut versucht

Eine Zustellung wird erneut versucht, wenn:

- Die HTTP-Anfrage **nicht abgeschlossen wird** (DNS-Fehler, Verbindung abgelehnt, Timeout).
- Der HTTP-Antwortcode ein beliebiger Nicht-2xx-Status ist, der nicht in der konfigurierten Liste **Statuscodes ohne Wiederholung** enthalten ist.

Eine Zustellung wird **nicht erneut versucht**, wenn:

- Der Antwortcode `2xx` ist (Erfolg).
- Der Antwortcode in der konfigurierten Liste **Statuscodes ohne Wiederholung** enthalten ist. Standardmäßig ist diese Liste leer – jeder Nicht-2xx-Code wird erneut versucht.

### Konfigurieren der Statuscodes ohne Wiederholung

Das Webhook-Konfigurationsformular hat ein Feld **Statuscodes ohne Wiederholung** (Mehrfachwerte). Gängige Einträge:

- `410` - Gone. Ihre Endpoint-URL wurde dauerhaft verschoben oder die Ressource existiert nicht mehr. Ein erneuter Versuch würde nur beide Seiten unnötig belasten.
- `422` - Unprocessable Entity. Ihr Endpoint hat die Nutzlast verstanden, sie aber als ungültig abgelehnt. Ein erneuter Versuch mit derselben Nutzlast führt zur gleichen Antwort.
- `400` - Bad Request, im gleichen Sinne.

Wenn Sie hier einen Code hinzufügen, bedeutet das: Wenn der Endpoint diesen zurückgibt, markieren Sie die Zustellung als failed-terminal und stellen keine weiteren Wiederholungsversuche an.

### Wiederholungsplan

Ein Hintergrundworker läuft alle paar Sekunden und verarbeitet alle Zustellungen, deren nächster Versuchstermin überschritten ist.

Nach jedem Fehler wird die Zeit für den nächsten Versuch mit **linearem Backoff** nach hinten verschoben: die Wartezeit wächst als `60 seconds * attempt count` (also wartet Versuch 1 1 Minute, Versuch 2 2 Minuten und so weiter).

Nach 99 fehlgeschlagenen Versuchen (oder 3 in der lokalen Entwicklung) wird die Zustellung aufgegeben und aus der Warteschlange entfernt. Die Einträge im Zustellungsprotokoll bleiben jedoch bestehen und sind auf der Seite [Webhook-Zustellungsprotokolle](#webhook-logs) sichtbar, bis sie ablaufen.

### Idempotenz auf Ihrer Seite

Da wir Wiederholungen durchführen, muss Ihr Endpoint **idempotent sein**. Dieselbe `triggerId` (oder `approvalId`) kann mehr als einmal ankommen. Ihr Endpoint sollte:

- Einen eindeutigen Schlüssel verwenden (`triggerId` für Trigger-Ereignisse, `approvalId` für Genehmigungs-Ereignisse) als Dedup-Token.
- Doppelte Zustellungen problemlos akzeptieren (geben Sie beim zweiten Mal 200 zurück).

Ein nicht-idempotenter Endpoint wird irgendwann einige Zustellungen doppelt verarbeiten, besonders bei vorübergehenden Ausfällen, bei denen ein Timeout 30 Sekunden später erneut versucht wird, während die ursprüngliche Anfrage tatsächlich erfolgreich war.

### Reihenfolge

Zustellungen sind **nicht strikt geordnet**. Ein `trigger.succeeded` und ein nachgelagerter `approval.requested` (aus demselben Lauf) können in beliebiger Reihenfolge eintreffen, wenn einer erneut versucht wird und der andere nicht. Ihr Endpoint sollte keine kausale Reihenfolge voraussetzen.

Wenn Sie eine Reihenfolge benötigen, verwenden Sie die Zeitstempel – `occurredAt` im Envelope sowie das Trigger-/Approval-`createdAt` im Datenblock –, um die Reihenfolge auf Ihrer Seite wiederherzustellen.

### Bereinigung

Zustellungen werden aus der Warteschlange entfernt, sobald sie entweder erfolgreich sind oder das Versuchslimit erreichen. Die Plattform behält terminal fehlgeschlagene Zustellungen nicht in der Warteschlange selbst; der dauerhafte Nachweis jedes Versuchs befindet sich auf der Seite [Webhook-Zustellungsprotokolle](#webhook-logs).

### Wohin schauen, wenn Wiederholungsversuche fehlschlagen

Die Seite [Webhook-Zustellungsprotokolle](#webhook-logs) ist der Ort, um zu sehen, warum ein Webhook fehlschlägt. Häufige Ursachen:

- **DNS-Auflösungsfehler** – die URL ist falsch oder die Domain existiert nicht mehr.
- **TLS-Fehler** – das Zertifikat Ihres Endpoints ist ungültig oder abgelaufen.
- **Verbindung abgelehnt / Timeout** – Ihr Endpoint ist nicht erreichbar.
- **5xx-Antworten** – Ihr Endpoint ist erreichbar, hat aber einen Fehler. Der Antwortkörper (gekürzt) wird protokolliert.
- **4xx-Antworten** – Ihr Endpoint hat die Nutzlast abgelehnt. Wenn dies beabsichtigt ist, fügen Sie den Code zur Liste **Statuscodes ohne Wiederholung** hinzu.

### Einen fehlerhaften Webhook pausieren

Wenn ein Webhook kontinuierlich fehlschlägt, ist die sauberste Lösung, ihn zu löschen (oder vorübergehend die Ereignis-Abonnementliste zu leeren). Die Plattform deaktiviert fehlerhafte Webhooks nicht automatisch – sie versuchen weiter, bis die Zustellung aufgegeben wird.