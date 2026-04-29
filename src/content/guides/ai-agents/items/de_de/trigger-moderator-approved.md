Wird ausgelöst, wenn ein Moderator einen Kommentar genehmigt.

### Kontext, den der Agent erhält

- Den neu genehmigten Kommentar.
- Die **auslösende Benutzer-ID** - der Moderator, der genehmigt hat.
- Optionaler Thread-/Benutzerverlauf/Seitenkontext, wie konfiguriert.

### Wer löst dies aus

Eine Aktion eines menschlichen Moderators.

### Bemerkenswert

- Ein „genehmigter“ Kommentar ist in der FastComments-Terminologie ein **sichtbarer** Kommentar. Siehe [How Approvals Work](/guide-moderation.html#moderation-approvals) im Moderationshandbuch für die Unterscheidung zwischen genehmigt/nicht genehmigt und überprüft/nicht überprüft.
- Der Trigger wird bei Genehmigungs-**Übergängen** ausgelöst: Ein Kommentar, der von nicht genehmigt zu genehmigt wechselt, löst ihn aus; ein Kommentar, der bereits genehmigt war und erneut gespeichert wird, tut dies nicht.
- Für Mandanten, bei denen Kommentare standardmäßig automatisch genehmigt werden, wird dieser Trigger nur ausgelöst, wenn ein Moderator explizit einen zuvor versteckten Kommentar erneut genehmigt.

### Häufige Verwendungszwecke

- **Willkommen / Engagement** - ein Agent kann Erstkommentatoren in dem Moment antworten, in dem ein Moderator sie genehmigt, anstatt zum Veröffentlichungszeitpunkt.
- **Agentenübergreifende Koordination** - wenn ein anderer Agent den Kommentar zur Überprüfung markiert hatte, ist die Genehmigung das Signal, dass die menschliche Überprüfung abgeschlossen ist.
- **Prüfprotokoll** über [Webhooks](#webhooks-overview).

---