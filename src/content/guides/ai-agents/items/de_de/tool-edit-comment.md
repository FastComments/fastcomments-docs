Das Edit-Tool ermöglicht es dem Agenten, den Text eines bestehenden Kommentars zu ersetzen. Es ist in einer Weise destruktiv, wie es die meisten anderen Moderationstools nicht sind: es überschreibt vom Nutzer verfasste Inhalte. Verwenden Sie es nur in engen, klar abgegrenzten Fällen.

### Was es bewirkt

Der Agent übergibt eine Kommentar-ID und einen Ersatztext. Die Plattform schreibt den neuen Text in den Kommentar und zeichnet einen `TextChanged`-Eintrag im Audit-Log des Kommentars auf, der sowohl den alten als auch den neuen Text festhält. Das Original geht nie verloren - Moderatoren können lesen, was der Kommentar vor der Bearbeitung durch den Agenten gesagt hat.

Die Ersetzung durchläuft dieselbe Rendering-Pipeline wie eine menschliche Bearbeitung: Schimpfwortmaskierung, Erwähnungsparsing, Hashtag-Extraktion sowie Link-/Bildverarbeitung verhalten sich genau so, als hätte der ursprüngliche Autor den neuen Text eingereicht.

### Umfang

Wie jedes kommentarändernde Tool ist Edit auf die Allowlist des Triggers beschränkt - der Agent kann nur den Kommentar bearbeiten, auf den der Trigger ausgelöst wurde, dessen Elternkommentar oder einen anderen im Geltungsbereich befindlichen Kommentar aus demselben Trigger-Kontext. Ein Prompt-Injection-Versuch, "Kommentar XYZ bearbeiten", wobei XYZ nicht dazu gehört, wird serverseitig abgelehnt, bevor der executor läuft.

### Schleifen

Wenn der Agent einen Kommentar bearbeitet, löst die Plattform einen `COMMENT_EDIT`-Trigger aus, wie bei einer menschlichen Bearbeitung, unterdrückt jedoch **die Weiterleitung an andere Agenten**. Dies verhindert, dass sich zwei Agenten, die beide auf `COMMENT_EDIT` hören, gegenseitig aufeinander reagieren.

### Wann man es erlauben sollte

Für Agenten, die PII-Redaktion durchführen, oder für selbst-editierende Zusammenfassungs-/Digest-Agenten. Die meisten Moderationsagenten benötigen dieses Tool **nicht** - mark-spam, warn, and ban decken den typischen Lebenszyklus ab.

### Genehmigungen

**Erwägen Sie dringend, es hinter einer Genehmigung zu platzieren**, insbesondere während Sie Vertrauen in den Agenten aufbauen. Dass ein Agent die Worte eines Nutzers umschreibt, ist eine Maßnahme, die die Community bemerken und auf die sie reagieren wird, und reputationsmäßig ist sie schwerer wieder gutzumachen als eine Löschung.

### Siehe auch

- [Trigger: Comment Edited](#trigger-comment-edit) - der Trigger, der ausgelöst wird, wenn sich der Text eines Kommentars ändert.
- [Approval Workflow](#approval-workflow) - wie man das Tool hinter einer menschlichen Überprüfung absichert.