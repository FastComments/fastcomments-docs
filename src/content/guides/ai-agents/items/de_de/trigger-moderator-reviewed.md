Wird ausgelöst, wenn ein Moderator einen Kommentar als überprüft markiert.

### Kontext, den der Agent erhält

- Den Kommentar.
- Die **auslösende Benutzer-ID** - der Moderator, der überprüft hat.
- Optionaler Thread- / Benutzerverlauf- / Seitenkontext je nach Konfiguration.

### Wer löst dies aus

Eine Aktion eines menschlichen Moderators auf der Moderationsseite, im Kommentar-Widget oder über die API.

### Häufige Verwendungszwecke

- **Audit-Weiterleitung** über [Webhooks](#webhooks-overview).
- **Memory-Schreibvorgänge** - zeichne eine Memory-Notiz auf, dass dieser Kommentar von einem Menschen überprüft wurde, damit andere Agenten ihn nicht doppelt verarbeiten.

### Bemerkenswert

- "Reviewed" ist einer der Zustände der Moderationswarteschlange, die separat von "approved" und "spam" verfolgt werden. Ein Kommentar kann "approved-and-reviewed", "approved-but-not-reviewed" usw. sein. Siehe [Wie Genehmigungen funktionieren](/guide-moderation.html#moderation-approvals) im Moderationsleitfaden.
- Dieser Trigger tritt bei Mandanten mit vielen Moderatoren häufig auf. Abonniere selektiv und plane das Budget entsprechend.

---