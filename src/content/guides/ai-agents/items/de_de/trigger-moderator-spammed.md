Wird ausgelöst, wenn ein Moderator einen Kommentar als Spam markiert.

### Kontext, den der Agent erhält

- Der Kommentar mit dem Post-Action-Flag `Is Spam`.
- Die **auslösende Benutzer-ID** - der Moderator, der gehandelt hat.
- Optionaler Thread-/Benutzerverlauf-/Seitenkontext, wie konfiguriert.

### Wer löst dies aus

Eine menschliche Moderatoraktion. Vom Agenten gesetzte Spam-Markierungen (via [`mark_comment_spam`](#tools-overview)) lösen diesen Trigger **nicht** aus.

### Häufige Verwendungszwecke

- **Speicheraufzeichnung** - einen Agenten veranlassen, eine Gedächtnisnotiz über den spam-markierten Benutzer zu speichern (z. B. "früher wegen X vom Moderator als Spam markiert"), damit künftige Moderationsagenten Kontext haben.
- **Durchsetzung auf Benutzerebene** - Wenn ein Moderator einen Kommentar als Spam markiert, kann das für einen Agenten das Signal sein, ebenfalls eine Verwarnung oder eine kurze Sperre auszusprechen, vorbehaltlich einer Genehmigung.
- **Spiegelung in externen Systemen** über [Webhooks](#webhooks-overview).

---