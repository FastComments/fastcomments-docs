De Warn tool stuurt een privé-DM-waarschuwing naar een gebruiker over een specifieke reactie, en registreert tegelijkertijd de waarschuwing in gedeeld [agent memory](#agent-memory-system). De twee schrijfacties zijn atomair - de gebruiker ziet nooit een waarschuwing die niet ook geregistreerd is.

### Waarom het bestaat

Het escalatiebeleid van het platform is **eerst waarschuwen, alleen bannen als de gebruiker opnieuw de fout ingaat**. De Warn tool maakt dat beleid uitvoerbaar: het geeft de gebruiker de kans om bij te sturen, en de waarschuwingsregistratie is wat een toekomstige agent vindt wanneer het geheugen wordt doorzocht voordat een ban wordt overwogen.

De tool dedupliceert ook: als de agent al een waarschuwing heeft gegeven aan dezelfde gebruiker over dezelfde reactie, is een tweede waarschuwing een no-op. Dus een LLM die loopt of opnieuw afgaat op dezelfde reactie kan de gebruiker niet spammen met meerdere waarschuwingen.

### Wat er in de waarschuwing staat

Een kort bericht (gemaximeerd op 1000 tekens) dat aan de gebruiker wordt getoond als DM. Sterke waarschuwingen zijn:

- **Specifiek** - "Persoonlijke aanvallen op benoemde gebruikers zijn niet toegestaan in deze community" is beter dan "je reactie is gemarkeerd."
- **Kort** - maximaal een paar zinnen.
- **Uitvoerbaar** - vertel de gebruiker wat er moet veranderen. "Bewerk je reactie om de benoemde gebruiker te verwijderen, anders wordt deze verwijderd."

Je schrijft het bericht niet zelf; dat doet de agent, op basis van de [initial prompt](#personality-prompt) en [community guidelines](#community-guidelines). Jouw taak is het schrijven van een prompt die goede waarschuwingen oplevert.

### Wanneer je het toestaat

Voor elke agent met een moderatietaak. De Moderator template schakelt het standaard in.

### Goedkeuringen

Wordt minder vaak afgeschermd dan [Ban user](#tool-ban-user). Het is de moeite waard om het af te schermen tijdens de eerste weken van het leven van een agent, zodat je slechte waarschuwingen kunt opsporen voordat ze de deur uit gaan, maar de meeste beheerders verwijderen de beveiliging zodra de agent betrouwbare output produceert.

### Zie ook

- [Ban user](#tool-ban-user) - de volgende stap in escalatie.
- [Agent Memory System](#agent-memory-system) - waar waarschuwingsregistraties worden bewaard.