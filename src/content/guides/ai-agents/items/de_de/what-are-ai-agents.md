Ein **KI‑Agent** ist ein autonomer Arbeiter, der auf Ihren FastComments‑Mandanten beschränkt ist, Ereignisse in Ihrer Community beobachtet und in Ihrem Namen handelt.

Jeder Agent hat drei Dinge, die Sie steuern können:

1. **Eine Persönlichkeit.** Ein freier Text‑Prompt, der Ton, Rolle und Entscheidungsstil definiert („Sie sind ein freundlicher Community‑Begrüßer“, „Sie setzen die Community‑Regeln durch, neigen jedoch eher zu Warnungen als zu Sperrungen“ usw.).
2. **Ein oder mehrere Trigger.** Eine Liste von Ereignissen, die den Agenten aktivieren – ein neuer Kommentar, ein Kommentar, der eine Stimmen‑ oder Meldeschwelle überschreitet, eine Moderator‑Aktion, der erste Kommentar eines Benutzers auf der Seite und weitere. Die vollständige Liste finden Sie in [Trigger Events Overview](#triggers-overview).
3. **Eine Positivliste von Werkzeugen.** Was der Agent tun darf – einen Kommentar posten, abstimmen, anheften, sperren, als Spam markieren, einen Benutzer sperren, per Direktnachricht warnen, ein Abzeichen verleihen, E‑Mails senden, einen gemeinsamen Speicher speichern und durchsuchen. Die vollständige Liste finden Sie in [Allowed Tool Calls Overview](#tools-overview).

Wenn ein Trigger ausgelöst wird, erhält der Agent eine Kontextnachricht, die beschreibt, was passiert ist (der Kommentar, die Seite, optionaler Thread‑/Benutzer‑/Seiten‑Kontext) und wird mit seinem Anfangsprompt sowie Ihren Community‑Richtlinien versehen. Anschließend ruft er Werkzeuge auf, um zu handeln, und protokolliert bei jedem Aufruf eine Begründung und einen Vertrauenswert.

### Agenten laufen asynchron

Agenten **blockieren niemals die Benutzeraktion, die sie ausgelöst hat**. Ein Leser postet einen Kommentar, der Kommentar wird gespeichert und im Thread angezeigt, die Antwort wird zurückgegeben, und erst *danach* führt der Agent seine Verarbeitung aus – entweder sofort oder nach einer konfigurierten Verzögerung (siehe [Deferred Triggers](#trigger-deferred-delay)). Nichts, was der Agent tut, erhöht die Latenz der Benutzererfahrung.

### Warum sie verwenden

- **In großem Maßstab moderieren.** Offensichtlichen Spam markieren und wiederholte Verstöße sperren, ohne die Warteschlange rund um die Uhr zu überwachen.
- **Neue Kommentatoren begrüßen.** Auf Erstkommentatoren in Ihrem Stil antworten.
- **Den besten Inhalt hervorheben.** Substantielle Top‑Level‑Kommentare anheften, sobald sie eine Stimmen‑Schwelle überschreiten.
- **Ihre Richtlinien konsequent durchsetzen.** Den gleichen Richtlinientext auf jeden Grenz‑Kommentar anwenden.
- **Lange Threads zusammenfassen.** Neutrale Zusammenfassungen von mehrseitigen Diskussionen posten.

### Was Sie in Kontrolle hält

- **Trockenlauf‑Modus.** Jeder neue Agent wird im **Dry Run** ausgeliefert: Er verarbeitet Trigger, führt das Modell aus und protokolliert, was er *tun würde*, führt jedoch keine echten Aktionen aus. Siehe [Dry-Run Mode](#dry-run-mode).
- **Genehmigungen.** Jede Teilmenge von Aktionen kann hinter einer menschlichen Genehmigung zurückgehalten werden. Siehe [Approval Workflow](#approval-workflow).
- **Pro‑Agent‑ und pro‑Konto‑Budgets.** Strenge tägliche und monatliche Obergrenzen. Siehe [Budgets Overview](#budgets-overview).
- **Werkzeug‑Positivliste.** Nicht erlaubte Werkzeuge werden aus der Palette des Modells entfernt – der Agent kann sie buchstäblich nicht anfordern. Siehe [Allowed Tool Calls Overview](#tools-overview).
- **Audit‑Felder bei jeder Aktion.** Das Modell muss eine Begründung und einen Vertrauenswert enthalten. Beide erscheinen in der Ablauf‑Zeitleiste und bei jeder Genehmigung. Siehe [Run Detail View](#run-detail-view).
- **EU‑DSA‑Artikel 17.** In der EU‑Region werden vollständig automatisierte Sperrungen blockiert. Siehe [EU DSA Article 17 Compliance](#eu-dsa-compliance).
- **Kein Training mit Ihren Daten.** FastComments verwendet Anbieter, die nicht mit Ihren Prompts oder Kommentaren trainieren.

### Wo sie neben der menschlichen Moderation passen

Agenten und menschliche Moderatoren nutzen dieselbe Kommentarplattform: Agenten führen Aktionen über dieselben Kanäle aus (genehmigen, Spam, sperren, Abzeichen, anheften, sperren, schreiben) und diese Aktionen erscheinen in denselben [Comment Logs](/guide-moderation.html#comment-logs), derselben [Moderation Page](/guide-moderation.html#moderate-comments-page) und denselben Benachrichtigungsströmen. Agenten und Menschen sehen die Arbeit des jeweils anderen und können darauf reagieren – Moderator‑Aktionen sind selbst gültige Agent‑Trigger (siehe [Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) und verwandte).

Falls Sie ebenfalls Personen in der Warteschlange benötigen, bietet FastComments [Human Moderation Services](/guide-moderation.html#human-moderation-services) an: Unsere Moderatoren arbeiten zusammen mit Ihren Agenten, in Ihrem Dashboard, nach Ihren Richtlinien.