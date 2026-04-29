**Prompt verfeinern** ist der Workflow zum Bearbeiten des [Initial-Prompt](#personality-prompt) eines Agenten als Reaktion auf bestimmte Entscheidungen, denen Sie widersprechen. Er wird aus dem [Genehmigungs-Posteingang](#approval-workflow) gestartet.

### Wann verwenden

Wenn Sie immer wieder dieselbe Art von Ablehnung vorfinden — „der Agent will Leute wegen starker Sprache ohne Ziel sperren“ — ist der Prompt des Agenten der Hebel, um das zu beheben. Prompt verfeinern ist ein geführter Weg, um:

1. Eine bestimmte Genehmigung auszuwählen, die die fehlerhafte Entscheidung repräsentiert.
2. Den Prompt mit vollem Kontext dessen, was der Agent getan hat und warum, zu bearbeiten.
3. Den neuen Prompt für den Agenten zu speichern.

Das Ergebnis ist ein Agent, der voraussichtlich nicht mehr dieselbe Entscheidung treffen würde.

### Starten des Ablaufs

Vom Genehmigungs-Posteingang unter `/auth/my-account/ai-agent-approvals`:

1. Öffnen Sie eine **rejected** Genehmigung. Die Route lehnt strikt alles außer REJECTED ab — pending- und execution-failed-Genehmigungen sind nicht berechtigt.
2. Klicken Sie auf **Refine prompt**.

Sie landen in der Prompt-Refine-Benutzeroberfläche unter `/auth/my-account/ai-agent-approvals/:approvalId/refine-prompt`.

### Was die Seite anzeigt

- **Die Genehmigung** – der Agenten-`toolName` und die `justification` für die abgelehnte Entscheidung (das vollständige LLM-Transkript wird hier nicht angezeigt).
- **Der aktuelle Prompt** – der gespeicherte [Initial-Prompt](#personality-prompt) des Agenten.
- **Ein Feedback-Eingabefeld** – Sie geben **Feedback** ein, das beschreibt, was sich ändern soll (bis zu 2000 Zeichen). Das LLM generiert dann aus Ihrem Feedback den vorgeschlagenen neuen Prompt.
- **Einheitlicher Inline-Diff** – ein einzelner Inline-Diff zwischen dem aktuellen und dem vorgeschlagenen Prompt (rot für Entferntes, grün für Hinzugefügtes).

Der Genehmigungskontext bleibt oben angeheftet, sodass Sie sich beim Bearbeiten ständig auf „den Fall, den ich behebe“ beziehen können.

### Speichern

Beim Speichern wird das `initialPrompt`-Feld des Agenten aktualisiert. Frühere Durchläufe (und frühere Genehmigungen) werden nicht nachträglich neu ausgeführt — der neue Prompt wirkt sich nur auf zukünftige Auslöser aus. Wenn Sie überprüfen möchten, ob der neue Prompt das Problem behebt, führen Sie einen [Testlauf / Replay](#test-runs-replays) für die letzten 7 Tage durch und prüfen Sie, ob der neue Prompt die abgelehnte Genehmigung weiterhin erzeugen würde.

### Was der Ablauf nicht macht

- Er bearbeitet nicht die **Community-Richtlinien** – dieses Feld hat seinen eigenen Editor im Hauptbearbeitungsformular des Agenten.
- Er bearbeitet nicht **Trigger**, **erlaubte Tools** oder **Approval Gating** – diese bleiben im Hauptbearbeitungsformular.
- Er versioniert den Prompt nicht mit Rollback. Der vorherige Prompt wird nicht in einer separaten Verlaufssammlung gespeichert. Wenn Sie zurückrollen müssen, kopieren Sie den aktuellen Prompt vor der Bearbeitung in Ihr eigenes Nachverfolgungssystem.

### Warum man Prompt verfeinern mit Replay koppeln sollte

Das Bearbeiten eines Prompts ohne Testen des Ergebnisses ist Glaube ohne Prüfung. Der empfohlene Zyklus:

1. Eine Genehmigung ablehnen.
2. Den Prompt verfeinern.
3. Einen [Testlauf](#test-runs-replays) für die letzten 7 Tage durchführen.
4. Den **Deltas**-Tab ansehen. Hat der neue Prompt die schlechte Entscheidung von „would do“ zu „would not do“ verschoben? Hat er versehentlich auch gute Entscheidungen verschoben?
5. Iterieren.

Drei oder vier Durchläufe von Prompt verfeinern + Replay reichen normalerweise aus, um einen stabilen Prompt für einen Moderationsagenten zu erhalten.

### Alternative: Direkte Bearbeitung

Sie müssen Prompt verfeinern nicht verwenden – Sie können den Agenten auch einfach im Hauptbearbeitungsformular direkt bearbeiten. Der einzige Vorteil von Prompt verfeinern ist, dass ein spezifischer fehlerhafter Fall angeheftet wird, sodass Sie nicht aus den Augen verlieren, wofür Sie gerade eine Lösung erarbeiten.