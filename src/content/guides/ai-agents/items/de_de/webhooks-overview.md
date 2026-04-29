Agent-Webhooks sind HTTP-Callbacks, die von der Plattform ausgelöst werden, wenn ein Agentenlauf abgeschlossen ist oder sich der Status einer Genehmigung ändert. Verwenden Sie sie, um Agentenaktivität in Ihre eigenen Systeme weiterzuleiten — Moderations-Dashboards, Audit-Protokolle, Slack-Kanäle, Eskalationstools.

Konfiguriert unter dem **Webhooks**-Reiter auf der [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents).

### What gets delivered

Four event types:

- **`trigger.succeeded`** - ein Agentenlauf wurde erfolgreich abgeschlossen.
- **`trigger.failed`** - ein Agentenlauf ist fehlgeschlagen.
- **`approval.requested`** - eine Aktion wurde zur menschlichen Genehmigung in die Warteschlange gestellt.
- **`approval.decided`** - eine Genehmigung wurde genehmigt, abgelehnt oder deren Ausführung ist fehlgeschlagen.

Siehe [Webhook Events](#webhook-events) für welche Ereignisse wann ausgelöst werden, und [Webhook Payloads](#webhook-payloads) für das Schema jedes einzelnen.

### What's not delivered

- **Per-tool-action webhooks.** Ein Lauf, der `pin_comment` aufruft, löst keinen separaten Webhook für das Pin aus — die Aktion ist im `trigger.succeeded`-Payload des Laufs enthalten. Wenn Sie eine Zustellung pro Aktion wünschen, parsen Sie das `actions`-Array im Trigger-Payload.
- **Dropped triggers.** Ein Trigger, der nicht ausgeführt wird (Budget überschritten, falscher Scope), löst keinen Webhook aus. Drops sind nur auf der [Analytics page](#analytics-page) sichtbar.
- **Replay-produced triggers.** Testläufe lösen keine Webhooks aus. Siehe [Test Runs (Replays)](#test-runs-replays).

### Configuration

For each webhook you set:

- **URL** - der HTTPS-Endpunkt, an den POST-Anfragen gesendet werden.
- **Domain** - die Kommentar-Domain, auf die dieser Webhook angewendet wird (Ihr Mandant kann mehrere Domains hosten). `*` matched alle Domains; `*.example.com` ist ein Glob; eine exakte Domain matched nur genau diese.
- **Events** - welche der vier Ereignistypen abonniert werden sollen.
- **Agents** - leer für 'alle Agenten', oder eine Liste spezifischer Agenten-IDs zur Filterung.
- **Method** - POST oder PUT (Standard: POST).
- **No-retry status codes** - HTTP-Antwortcodes, die als endgültige Fehler behandelt werden sollten und nicht erneut versucht werden (z. B. 410 Gone, 422 Unprocessable). Siehe [Webhook Retries](#webhook-retries).

Mehrere Webhooks können sich auf dasselbe Ereignis abonnieren — jeder wird unabhängig in die Warteschlange gestellt und an seine eigene URL zugestellt.

### Per-domain matching

Ein gegebenes Ereignis wird an **jeden Webhook zugestellt, dessen `domain`-Feld mit der Domain des Kommentars übereinstimmt**. Die Übereinstimmung verwendet ein einfaches Glob-Muster:

- Exact: `example.com` matched nur `example.com`.
- Wildcard star: `*` matched jede Domain.
- Subdomain glob: `*.example.com` matched `blog.example.com`, `forum.example.com`, aber nicht `example.com` selbst.

Die Domain in einem Payload ist das erste nicht-null Ergebnis von: der `domain` des Kommentars, der gegen die Domain-Konfiguration Ihres Mandanten geparsten URL oder der `Page`-Lookup durch `urlId`.

### Per-agent filtering

Das **Agents**-Feld erlaubt einem Webhook, sich nur für bestimmte Agenten zu abonnieren. Leer bedeutet 'alle Agenten'. Wenn nicht leer, wird der Webhook nur für die Agenten in der Liste ausgelöst.

Das ist nützlich, wenn Sie einen Webhook für Moderationsereignisse und einen anderen für Engagement-Ereignisse haben, die jeweils an unterschiedliche nachgelagerte Systeme weitergeleitet werden.

### Test send

Die Webhook-Konfigurationsoberfläche hat eine **Test senden**-Schaltfläche, die eine gefälschte Nutzlast an die URL postet, damit Sie Konnektivität, Signierung und den Antwortcode Ihres Endpunkts überprüfen können, ohne auf ein echtes Ereignis warten zu müssen.

### Delivery logs

Jede Zustellung (und jeder Retry) landet auf der [Webhook Delivery Logs](#webhook-logs)-Seite, damit Sie sehen können, was auf der Leitung passiert ist.

### Signing

Jeder Webhook wird mit HMAC-SHA256 unter Verwendung des API-Secrets Ihres Mandanten signiert. Siehe [Webhook Signing](#webhook-signing).

### Eligibility

Agent-Webhooks setzen eine gültige Abrechnung für den Mandanten voraus. Sie verwenden die gleiche Signatur-/Secret-Infrastruktur wie Ihre bestehenden Kommentar-Webhooks — wenn Sie Kommentar-Webhooks bereits integriert haben (siehe den [Webhooks guide](/guide-webhooks.html)), ist die Integration der Agent-Webhooks gleich aufgebaut, bietet aber neue Ereignistypen.