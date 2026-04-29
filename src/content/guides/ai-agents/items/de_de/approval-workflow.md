Eine **Genehmigung** ist ein in die Warteschlange gestellter Tool-Aufruf, der von einem Menschen genehmigt oder abgelehnt werden muss, bevor die Plattform ihn ausführt.

### Konfiguration von Genehmigungen

Im Bearbeitungsformular des Agenten listet der Abschnitt **Genehmigungen** jedes Tool auf, das der Agent aufrufen darf (die allowlist) und erlaubt es Ihnen, diejenigen anzukreuzen, die von einem Menschen überprüft werden müssen. Nicht angekreuzte Tools werden sofort ausgeführt. Angeklickte Tools werden in die Warteschlange gestellt.

Nicht erlaubte Tools werden *komplett abgelehnt*, nicht in die Warteschlange gestellt – Genehmigungen gelten nur für Tools, die ansonsten erlaubt sind.

### Was passiert, wenn eine gesperrte Aktion ausgelöst wird

1. Der Agent wählt einen Tool-Aufruf (z. B. `ban_user`) mit Argumenten, Begründung und Vertrauenswert.
2. Anstatt ihn auszuführen, stellt die Plattform eine Genehmigung mit dem Zustand `PENDING` in die Warteschlange, inklusive Tool-Name, Argumenten, Begründung, Vertrauenswert und einem Kontext-Snapshot, der den Auslöser beschreibt.
3. Benachrichtigungen gehen an die Prüfer raus (siehe [Benachrichtigungen zu Genehmigungen](#approval-notifications)).
4. Der Lauf des Agenten wird abgeschlossen und aufgezeichnet – die Aktion wird mit **Ausstehende Genehmigung** in der [Detailansicht der Ausführung](#run-detail-view) angezeigt.

### Genehmigungen prüfen

Der Posteingang für Genehmigungen unter [my-account/ai-agent-approvals](https://fastcomments.com/auth/my-account/ai-agent-approvals) listet ausstehende, genehmigte, abgelehnte und fehlgeschlagene Ausführungs-Genehmigungen. Für jede:

- **Tool-Name und Argumente** – genau das, was der Agent tun will.
- **Agentenbegründung** – die Begründung, die der Agent geliefert hat.
- **Vertrauenswert** – die vom Agenten selbst eingeschätzte Vertrauenswahrscheinlichkeit, 0.0 bis 1.0.
- **Kontext-Snapshot** – der Kommentar, die Seite und der Benutzer, auf die sich die Aktion richtet.

Zwei Buttons: **Genehmigen** und **Ablehnen**. Genehmigen führt das Tool tatsächlich aus; Ablehnen verwirft es.

### Status einer Genehmigung

Eine Genehmigung durchläuft diese Zustände:

| Zustand | Bedeutung |
|---|---|
| `PENDING` | Wartet auf eine menschliche Entscheidung. |
| `APPROVED` | Ein Mensch hat genehmigt und die Aktion wurde ausgeführt. |
| `REJECTED` | Ein Mensch hat abgelehnt. Die Aktion wurde nicht ausgeführt. |
| `EXECUTION_FAILED` | Ein Mensch hat genehmigt, aber die Ausführung ist fehlgeschlagen (z. B. war der Zielkommentar bereits gelöscht). |
| `EXECUTING` | Transient: ein Mensch hat auf Genehmigen geklickt und die Aktion läuft. Wird verwendet, um gleichzeitige Genehmigungs-Klicks zu serialisieren, sodass ein Tool mit echten Seiteneffekten nie zweimal läuft. |

Der Zustand `EXECUTING` ist wichtig, wenn zwei Prüfer gleichzeitig auf Genehmigen klicken – einer „gewinnt“, der andere sieht, dass die Genehmigung bereits weitergegangen ist.

### Was bereinigt wird

Ausstehende Genehmigungen bleiben ausstehend, bis entschieden wird. Sie laufen automatisch **90 Tage** nach der Erstellung ab. Genehmigungen in jedem anderen Zustand werden ebenfalls nach 90 Tagen zur Speicherhygiene gelöscht.

Die Felder „decided by“ / „decided at“ / „executed at“ / „execution result“ der Genehmigung werden ausgefüllt, während die Genehmigung die Zustände durchläuft – alles sichtbar in der Detailansicht des Posteingangs.

### Webhook-Integration

Zwei Webhook-Ereignisse werden ausgelöst, wenn Genehmigungen sich ändern:

- **`approval.requested`** - bei Einfügen in `PENDING`.
- **`approval.decided`** - beim Übergang zu `APPROVED`, `REJECTED` oder `EXECUTION_FAILED`.

Beide werden wie alle anderen Webhooks signiert. Siehe [Webhook-Ereignisse](#webhook-events) und [Webhook-Payloads](#webhook-payloads).

### Härtung: known-tool gate

Genehmigungen verweigern das Enqueuing jeglicher Tool-Namen, die nicht als Agent-Tool erkannt werden. Dies ist eine Defense-in-Depth-Prüfung: Selbst wenn ein zukünftiger Codepfad einen LLM-abgeleiteten Tool-Namen in den Genehmigungsfluss übergibt, kann diese Zeichenkette niemals als wartende Genehmigung landen. Die Menge der bekannten Tool-Namen ist dieselbe Menge, die in der [Tools-Referenz](#tools-overview) aufgeführt ist.

### Gängige Gate-Muster

- **Brandneuer Moderationsagent** – sperren Sie `ban_user`, `mark_comment_spam`, `mark_comment_approved`, `send_email`. Beobachten Sie den Posteingang für ein paar Wochen und entfernen Sie dann die Sperre bei Tools mit niedriger Fehlerquote.
- **Langfristiger Moderationsagent** – behalten Sie `ban_user` und alle irreversiblen Aktionen (`deleteAllUsersComments`, `banIP`) dauerhaft gesperrt.
- **EU-Region** – `ban_user` ist durch Artikel 17 aktiviert, unabhängig davon, was Sie ankreuzen. Siehe [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Was Genehmigungen **nicht** tun

- Sie verzögern nicht die anderen Tool-Aufrufe des Agenten. Der Lauf des Agenten kann mehrere Tools aufrufen, und nur die gesperrten werden in die Warteschlange gestellt – der Rest wird wie gewohnt ausgeführt.
- Sie rollen den Lauf des Agenten nicht zurück, wenn ein Mensch ablehnt. Der nicht gesperrte Teil des Laufs ist bereits abgeschlossen.