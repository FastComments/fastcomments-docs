Dies ist der Fünf-Minuten-Weg von „wir haben AI Agents" zu „ein Agent reagiert auf echten Traffic, abgesichert durch Freigaben". Wenn Sie die ausführliche Variante möchten, verlinkt jeder Schritt auf die Seite, die ihn im Detail behandelt.

### 1. Öffnen Sie die AI Agents-Seite

Gehen Sie in Ihrem Konto zu [AI Agents](https://fastcomments.com/auth/my-account/ai-agents). Wenn Sie diese Seite zum ersten Mal aufrufen, sehen Sie entweder:

- Einen leeren Zustand mit den Schaltflächen **Browse templates** und **Start from scratch** (es stehen Agents zum Erstellen zur Verfügung), oder
- Eine Upsell-Seite, wenn Ihr Plan keine Agents enthält - siehe [Plans and Eligibility](#plans-and-eligibility).

### 2. Wählen Sie eine Starter-Vorlage

Klicken Sie auf **Browse templates**. Wählen Sie eine der folgenden:

- [Moderator](#template-moderator) - prüft gemeldete oder neue Kommentare, warnt Erstkommentierer und sperrt erst nach einer Warnung.
- [Welcome Greeter](#template-welcome-greeter) - antwortet Erstkommentierern.
- [Top Comment Pinner](#template-top-comment-pinner) - pinnt inhaltlich wertvolle Kommentare, sobald sie eine Stimmschwelle überschreiten.
- [Thread Summarizer](#template-thread-summarizer) - veröffentlicht eine neutrale Zusammenfassung bei langen Threads.

Jede Vorlage landet auf einem vorausgefüllten Bearbeitungsformular, bei dem **Status: Dry Run** bereits ausgewählt ist.

### 3. Prüfen und speichern

Nehmen Sie im Bearbeitungsformular mindestens Folgendes vor:

- **Internal name.** Ein kurzer Bezeichner, der in Admin-Dashboards verwendet wird.
- **Display name.** Was öffentlich erscheint, wenn der Agent einen Kommentar postet.
- **Initial prompt.** Passen Sie den Prompt der Vorlage an Ihre Tonalität und Ihre konkreten Regeln an.
- **Approvals.** Setzen Sie Häkchen bei den Aktionen, die vor ihrer Ausführung eine menschliche Prüfung erfordern sollen. Wir empfehlen mindestens `ban_user` für jeden moderationsähnlichen Agent. Siehe [Approval Workflow](#approval-workflow).

Klicken Sie auf **Save agent**.

### 4. Beobachten Sie den Agent im Dry-Run

Der Agent ist jetzt im **Dry Run** aktiv. Er empfängt seine Trigger, ruft das Modell auf und zeichnet Aktionen auf der [Run History](#run-history)-Seite auf - jeweils mit dem **Dry Run**-Badge - führt aber keine echten Aktionen aus. Sehen Sie sich einige der Ausführungsdetails an (siehe [Run Detail View](#run-detail-view)) und beachten Sie:

- Die Aktionen, die der Agent ausgewählt hat.
- Die Begründung und Konfidenz jeder Aktion.
- Das vollständige LLM-Transkript.

Wenn der Agent Entscheidungen trifft, mit denen Sie nicht einverstanden sind, passen Sie den Initial-Prompt an oder setzen Sie Häkchen bei weiteren Approvals.

### 5. Führen Sie einen Test gegen frühere Kommentare durch

Klicken Sie auf der Agents-Listenseite in der Zeile des Agents auf **Test run**. Das Formular enthält ein einzelnes numerisches Feld **Days** (1 bis 90). Die Stichprobengröße und das harte Limit der ausgewerteten Kommentare werden nur informativ angezeigt - sie werden serverseitig berechnet, nicht vom Nutzer festgelegt. Die Wiederholung läuft gegen historische Kommentare, ohne echte Aktionen auszuführen, und berichtet, was der Agent **getan hätte** im Vergleich zu dem, was tatsächlich geschah (wurde der Kommentar später freigegeben, als Spam markiert, gelöscht usw.). Siehe [Test Runs (Replays)](#test-runs-replays).

### 6. Auf Enabled umschalten

Wenn Sie mit dem Dry-Run- und Replay-Ergebnis zufrieden sind, bearbeiten Sie den Agent und ändern Sie den **Status** auf **Enabled**. Ab jetzt greifen echte Aktionen. Die Run-History-Seite zeigt nun Live-Läufe ohne das Dry-Run-Badge, und jede Aktion, die Sie zur Freigabe markiert haben, erscheint im [Approvals-Posteingang](#approval-workflow).

### Wie geht es weiter

- Richten Sie [Budgets](#budgets-overview) und [Budget Alerts](#budget-alerts) ein.
- Konfigurieren Sie [Webhooks](#webhooks-overview), wenn externe Systeme auf Agent-Ereignisse reagieren sollen.
- Fügen Sie [Community Guidelines](#community-guidelines) hinzu, damit die Entscheidungen des Agents mit Ihrer schriftlichen Richtlinie übereinstimmen.