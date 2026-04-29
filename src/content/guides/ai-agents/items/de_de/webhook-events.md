Es gibt vier Agent-Webhook-Ereignistypen. Jedes Ereignis hat einen numerischen Enum-Wert (verwendet in Payloads) und einen kanonischen String-Namen (verwendet im `event`-Envelope-Feld und im `X-FastComments-Agent-Event` HTTP-Header).

| Event name | Enum | Löst aus, wenn |
|---|---|---|
| `trigger.succeeded` | 0 | Ein Agent-Lauf wird mit dem Status `SUCCESS` abgeschlossen. |
| `trigger.failed` | 1 | Ein Agent-Lauf wird mit dem Status `ERROR` abgeschlossen. |
| `approval.requested` | 2 | Eine Genehmigung wird in den Zustand `PENDING` eingestellt. |
| `approval.decided` | 3 | Eine Genehmigung wechselt zu `APPROVED`, `REJECTED` oder `EXECUTION_FAILED`. |

### `trigger.succeeded`

Wird ausgelöst, nachdem der Lauf des Agents ohne Fehler abgeschlossen ist. Das `data`-Feld des Payloads enthält:

- `triggerId` - die eindeutige Lauf-ID.
- `triggerType` - das [Trigger-Grund-Enum](#triggers-overview), das den Lauf gestartet hat.
- `status` - `SUCCESS` (string).
- `tokensUsed` - in diesem Lauf verbrauchte Tokens.
- `wasDryRun` - true, wenn der Agent im [Trockenlaufmodus](#dry-run-mode) war.
- `actions` - Array von `TenantAgentAction`-Einträgen (siehe [Webhook-Nutzlasten](#webhook-payloads)).
- `commentId`, `url`, `urlId` - falls der Trigger diese hatte.

Wenn der Lauf null Aktionen ausgeführt hat, ist das `actions`-Array leer – dies ist ein erfolgreicher Lauf im Sinne von „der Agent entschied, nichts zu tun“, was nützlich zu wissen ist.

### `trigger.failed`

Wird ausgelöst, wenn ein Lauf mit einem Fehler endet. Gleiche Payload-Struktur wie bei `trigger.succeeded`, mit `status: 'ERROR'` und einem zusätzlichen Feld `errorMessage`, das beschreibt, was schiefgelaufen ist. Mögliche Fehler umfassen Fehler bei LLM-Aufrufen, Fehler bei der Tool-Weiterleitung und Budgeterschöpfung während des Laufs.

`actions` kann weiterhin Einträge für Tool-Aufrufe enthalten, die vor dem Fehler abgeschlossen wurden.

### `approval.requested`

Wird in dem Moment ausgelöst, in dem eine Genehmigung in den Status `PENDING` eingereiht wird. Der Payload enthält:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - die Argumente des Tools, **unverändert weitergereicht** aus dem LLM-Aufruf. Die Form ist pro Tool verschieden und kein stabiles öffentliches Contract-Format – das Schema kann sich ändern, wenn neue Tools hinzugefügt werden.
- `createdAt`.
- `justification`, `confidence` - falls der Agent sie angegeben hat.
- `contextSnapshot` - der Kommentar-/Seitenkontext, auf den sich die Genehmigung bezieht.

Nützlich, um ausstehende Genehmigungen in einen Chat-Ops-Kanal weiterzuleiten: Ein Slack-Bot, der auf `approval.requested` abonniert ist, kann die Aktion und die Begründung in einen Moderationskanal posten, um sie auf einen Blick zu prüfen.

### `approval.decided`

Wird ausgelöst, wenn eine Genehmigung aus dem Zustand `PENDING` heraus bewegt wird. Der Payload enthält:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED` oder `EXECUTION_FAILED`.
- `decidedBy` - die Benutzer-ID des Moderators, der entschieden hat.
- `decidedAt` - Zeitpunkt der Entscheidung.
- `executedAt` - falls APPROVED, wann die Plattform die genehmigte Aktion ausgeführt hat.
- `executionResult` - falls APPROVED, ein String, der das Ergebnis des Ausführenden beschreibt.
- `contextSnapshot` - der Kommentar-/Seitenkontext.

Dieses Ereignis umfasst alle Entscheidungsresultate:

- **Approved + executed cleanly** -> `status: APPROVED`, `executedAt` gesetzt, `executionResult` ist die Erfolgsmeldung.
- **Approved + executor failed** -> `status: EXECUTION_FAILED`, `executedAt` gesetzt, `executionResult` beschreibt den Fehlschlag.
- **Rejected** -> `status: REJECTED`, `executedAt` ist null, `executionResult` ist null.

### Header

Jede Zustellung enthält einen HTTP-Header `X-FastComments-Agent-Event` mit dem kanonischen String-Namen des Ereignisses (`trigger.succeeded` usw.). Nützlich, wenn Ihr Endpunkt eine einzige URL ist, die mehrere Ereignistypen verarbeitet.

### Siehe auch

- [Webhook-Nutzlasten](#webhook-payloads) für vollständige pro-Ereignis-Payload-Schemata.
- [Webhook-Signierung](#webhook-signing) für das HMAC-Schema.
- [Webhook-Wiederholungen](#webhook-retries) für die Zustellsemantik.