Udløses, når en kommentar automatisk markeres som spam af FastComments' indbyggede spammotor - **ikke** af en moderator og ikke af en anden agent.

### Den kontekst agenten modtager

- Den kommentar, der automatisk er markeret som spam.
- Valgfri tråd / brugerhistorik / sidekontekst som konfigureret.

### Hvem udløser dette

Platformens spam-pipeline. Se [Spamdetektion](/guide-moderation.html#spam-detection) i moderationsguiden for flere detaljer.

### Almindelige anvendelser

- **Anden-gangs moderering** - spammotoren har høj recall, men upræcis præcision; en agent trænet i din specifikke fællesskabsstil kan fange falske positiver. Agenten kan foretage et kald for at fjerne flagget fra en fejlagtigt klassificeret kommentar.
- **Automatiseret ophævelse af udelukkelse** - hvis din tenant aggressivt spam-udelukker nye konti, kan en agent på denne trigger gennemgå og rydde åbenlyse falske positiver, før et menneske nogensinde ser dem.

### Bemærk

- Triggeren udløses **ikke** for spam markeret af en moderator (brug [Trigger: Moderator-markeret spam](#trigger-moderator-spammed)) eller for spam markeret af en anden agent.
- En kommentar, der automatisk markeres som spam og senere markeres som Ikke Spam af en moderator, udløser ikke triggeren igen.
- At abonnere på denne trigger er mest nyttigt i tenants, hvor motorens auto-spam-tilstand er aktiveret under Moderationsindstillinger. Ellers vil triggeren ikke udløses.