Agent-Webhooks sind HTTP-Callbacks, die von der Plattform ausgelöst werden, wenn ein Agent-Run abgeschlossen ist oder sich der Status einer Genehmigung ändert. Verwenden Sie sie, um Agent-Aktivitäten in Ihre eigenen Systeme weiterzuleiten – Moderations-Dashboards, Audit-Logs, Slack-Kanäle, Eskalationstools.

Konfiguriert werden sie unter dem Tab **Webhooks** auf der [AI Agents-Seite](https://fastcomments.com/auth/my-account/ai-agents).

### Was zugestellt wird

Vier Ereignistypen:

- **`trigger.succeeded`** – ein Agent-Run wurde erfolgreich abgeschlossen.
- **`trigger.failed`** – ein Agent-Run ist fehlgeschlagen.
- **`approval.requested`** – eine Aktion wurde zur menschlichen Genehmigung in die Warteschlange gestellt.
- **`approval.decided`** – eine Genehmigung wurde genehmigt, abgelehnt oder die Ausführung ist fehlgeschlagen.

Siehe [Webhook Events](#webhook-events), um zu erfahren, welche Ereignisse wann ausgelöst werden, und [Webhook Payloads](#webhook-payloads) für das Schema der einzelnen Ereignisse.

### Was nicht zugestellt wird

- **Webhooks pro Tool-Aktion.** Ein Run, der `pin_comment` aufruft, löst keinen separaten Webhook für das Pinning aus – die Aktion ist im `trigger.succeeded`-Payload des Runs enthalten. Wenn Sie eine Zustellung pro Aktion wünschen, parsen Sie das `actions`-Array im Trigger-Payload.
- **Verworfene Trigger.** Ein Trigger, der nicht versendet wird (Budget überschritten, falscher Scope), löst keinen Webhook aus. Verworfene Trigger sind nur auf der [Analytics-Seite](#analytics-page) sichtbar.
- **Durch Replays erzeugte Trigger.** Test-Runs lösen keine Webhooks aus. Siehe [Test Runs (Replays)](#test-runs-replays).

### Konfiguration

Für jeden Webhook legen Sie fest:

- **URL** – der HTTPS-Endpunkt, an den gepostet wird.
- **Domain** – die Kommentar-Domain, für die dieser Webhook gilt (Ihr Tenant kann mehrere Domains hosten). `*` passt auf alle Domains; `*.example.com` ist ein Glob; eine exakte Domain passt nur auf diese eine.
- **Events** – welche der vier Ereignistypen abonniert werden.
- **Agents** – leer für „alle Agents" oder eine Liste spezifischer Agent-IDs zum Filtern.
- **Method** – POST oder PUT (Standard: POST).
- **No-retry status codes** – HTTP-Antwortcodes, die als endgültige Fehler behandelt und nicht erneut versucht werden sollen (z. B. 410 Gone, 422 Unprocessable). Siehe [Webhook Retries](#webhook-retries).

Mehrere Webhooks können dasselbe Ereignis abonnieren – jeder wird unabhängig in die Warteschlange gestellt und an seine eigene URL zugestellt.

### Domain-spezifisches Matching

Ein bestimmtes Ereignis wird an **jeden Webhook zugestellt, dessen `domain`-Feld mit der Domain des Kommentars übereinstimmt**. Das Matching verwendet einen einfachen Glob:

- Exakt: `example.com` passt nur auf `example.com`.
- Wildcard-Sternchen: `*` passt auf jede Domain.
- Subdomain-Glob: `*.example.com` passt auf `blog.example.com`, `forum.example.com`, aber nicht auf `example.com` selbst.

Die Domain im Payload ist das erste Nicht-Null-Ergebnis aus: der `domain` des Kommentars, der gegen die Domain-Konfiguration Ihres Tenants geparsten URL oder dem `Page`-Lookup per `urlId`.

### Filterung nach Agent

Das Feld **Agents** ermöglicht es einem Webhook, nur bestimmte Agents zu abonnieren. Leer bedeutet „alle Agents". Wenn es nicht leer ist, wird der Webhook nur für Agents in der Liste ausgelöst.

Dies ist nützlich, wenn Sie einen Webhook für Moderationsereignisse und einen weiteren für Engagement-Ereignisse haben, die beide an verschiedene nachgelagerte Systeme weitergeleitet werden.

### Test send

Die Webhook-Konfigurations-UI hat eine Schaltfläche **Test send**, die ein gefälschtes Payload an die URL postet, damit Sie Konnektivität, Signierung und den Antwortcode Ihres Endpunkts überprüfen können, ohne auf ein echtes Ereignis warten zu müssen.

### Zustellungs-Logs

Jede Zustellung (und jeder Retry) landet auf der Seite [Webhook Delivery Logs](#webhook-logs), damit Sie sehen können, was über die Leitung gegangen ist.

### Signierung

Jeder Webhook wird mit HMAC-SHA256 unter Verwendung des API-Secrets Ihres Tenants signiert. Siehe [Webhook Signing](#webhook-signing).

### Voraussetzungen

Agent-Webhooks erfordern eine gültige Abrechnung für den Tenant. Sie verwenden dieselbe Signierungs-/Secret-Infrastruktur wie Ihre bestehenden Kommentar-Webhooks – wenn Sie bereits Kommentar-Webhooks integriert haben (siehe den [Webhooks-Leitfaden](/guide-webhooks.html)), hat die Agent-Webhook-Integration dieselbe Struktur mit neuen Ereignistypen.