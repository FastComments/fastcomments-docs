Wird ausgelöst, wenn ein Moderator einem Benutzer ein Abzeichen verleiht.

### Kontext, den der Agent erhält

- Die **Abzeichen-ID** des verliehenen Abzeichens.
- Die **auslösende Benutzer-ID** - der Moderator, der das Abzeichen verliehen hat.
- Optionaler Thread-/Benutzerverlauf-/Seitenkontext, je nach Konfiguration.

Die Auslöse-Stelle enthält **nicht** die `commentId` im Trigger-Payload, selbst wenn das Abzeichen im Zusammenhang mit einem bestimmten Kommentar vergeben wurde.

### Wer löst dies aus

Eine Aktion eines menschlichen Moderators.

### Bemerkenswert

- Es wird nur die Abzeichen-ID übermittelt; der Agent erhält nicht die Abzeichen-Metadaten (Name, Bild). Wenn der Agent bestimmen muss, *welches* Abzeichen vergeben wurde, bette diesen Kontext in die [Initialaufforderung](#personality-prompt) oder die [Community-Richtlinien](#community-guidelines) ein.
- Der Trigger wird einmal pro Abzeichenvergabe ausgelöst, nicht pro Benutzer. Wenn dasselbe Abzeichen einem Benutzer zweimal verliehen wird, wird der Trigger auch zweimal ausgelöst (jede Verleihung ist ein eigenständiges Ereignis).

### Häufige Anwendungsfälle

- **Gegenseitige Anerkennung** - Ein Agent kann eine Antwort wie "Danke für den großartigen Beitrag" posten, wenn ein bestimmtes Abzeichen verliehen wird.
- **Externer Anerkennungs-Workflow** über [Webhooks](#webhooks-overview) - Abzeichenverleihungen in Ihr eigenes Nutzer-Engagement-System spiegeln.
- **Gedächtnisaufzeichnung** - Hinweise wie „Dieser Benutzer ist ein anerkannter Mitwirkender“, die künftige Moderationsagenten bei ihren Entscheidungen stärker gewichten sollten.

---