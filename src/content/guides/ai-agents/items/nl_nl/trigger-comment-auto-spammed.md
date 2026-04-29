Wordt geactiveerd wanneer een reactie automatisch als spam wordt gemarkeerd door de ingebouwde spam-engine van FastComments - **niet** door een moderator en niet door een andere agent.

### Context die de agent ontvangt

- De automatisch als spam gemarkeerde reactie.
- Optionele thread / gebruikersgeschiedenis / pagina-context zoals geconfigureerd.

### Wie activeert dit

De spam-pijplijn van het platform. Zie [Spamdetectie](/guide-moderation.html#spam-detection) in de moderatiegids voor meer details.

### Veelvoorkomende toepassingen

- **Tweede controle** - de spam-engine heeft hoge recall maar onvolmaakte precision; een agent getraind op de specifieke stijl van uw community kan vals-positieven opsporen. De agent kan een ten onrechte geclassificeerde reactie ontmarkeren.
- **Geautomatiseerd deblokkeren** - als uw tenant nieuwe accounts agressief als spam blokkeert, kan een agent op deze trigger duidelijke vals-positieven beoordelen en wissen voordat een mens ze ziet.

### Opmerkelijk

- De trigger wordt **niet** geactiveerd voor door een moderator gemarkeerde spam (gebruik [Trigger: Door moderator gemarkeerde spam](#trigger-moderator-spammed)) noch voor door een andere agent gemarkeerde spam.
- Een reactie die automatisch als spam is gemarkeerd en vervolgens later door een moderator als Not Spam wordt aangemerkt, activeert de trigger niet opnieuw.
- Abonneren op deze trigger is het meest nuttig in tenants waar de engine's auto-spammodus is ingeschakeld onder Moderation Settings. Anders zal de trigger niet afgaan.

---