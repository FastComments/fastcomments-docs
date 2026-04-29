Wird ausgelöst, wenn ein Kommentar gesperrt wird.

### Kontext, den der Agent erhält

- Der gesperrte Kommentar.
- Optionaler Thread-/Benutzerverlauf-/Seitenkontext, wie konfiguriert.

### Wer löst dies aus

- Ein Moderator, der die Sperraktion auf der Moderationsseite oder im Kommentar-Widget verwendet.

### Typische Anwendungsfälle

- **Reviewer benachrichtigen** - ein Sperre-Ereignis folgt oft auf einen hitzigen Thread; ein Webhook an Ihren Moderations-Slack-Kanal kann es Menschen ermöglichen, den Rest zu übernehmen.
- **Durchsetzung einer Abkühlzeit** - plane einen [verzögerten Trigger](#trigger-deferred-delay) auf einem separaten Agenten, der 24 Stunden nach der Sperrung prüft, ob wieder freigegeben werden soll.

### Gegenstück

Siehe [Trigger: Kommentar entsperrt](#trigger-comment-unlock) für das Gegenstück.

---