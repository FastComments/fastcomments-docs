De Warn tool stuurt een privé-DM-waarschuwing naar een gebruiker over een specifiek commentaar, en legt tegelijkertijd de waarschuwing vast in het gedeelde [agent memory](#agent-memory-system). De twee schrijfhandelingen zijn atomair - de gebruiker ziet nooit een waarschuwing die niet ook geregistreerd is.

### Why it exists

Het escalatiebeleid van het platform is **eerst waarschuwen, alleen verbannen als de gebruiker opnieuw overtreedt**. De Warn tool maakt dat beleid uitvoerbaar: het geeft de gebruiker de kans zich te corrigeren, en het waarschuwingrecord is wat een toekomstige agent aantreft wanneer hij het geheugen doorzoekt voordat hij een ban overweegt.

De tool zorgt ook voor deduplicatie: als de agent al een waarschuwing heeft uitgegeven aan dezelfde gebruiker over hetzelfde commentaar, is een tweede waarschuwing een no-op. Dus een LLM die blijft loopen of opnieuw afvuurt op hetzelfde commentaar kan de gebruiker niet spammen met meerdere waarschuwingen.

### What goes in the warning

Een kort bericht (maximaal 1000 tekens) dat aan de gebruiker als DM wordt getoond. Sterke waarschuwingen zijn:

- **Specific** - "Personal attacks on named users are not allowed in this community" verslaat "your comment was flagged."
- **Short** - maximaal een paar zinnen.
- **Actionable** - vertel de gebruiker wat te veranderen. "Please edit your comment to remove the named user, or it will be removed."

Je schrijft het bericht niet zelf; de agent doet dat, op basis van de [initial prompt](#personality-prompt) en de [community guidelines](#community-guidelines). Je taak is om een prompt te schrijven die goede waarschuwingen oplevert.

### When to allow it

Voor elke moderatieachtige agent. Het Moderator template schakelt het standaard in.

### Approvals

Minder vaak achter een goedkeuring geplaatst dan [Ban user](#tool-ban-user). Het is de moeite waard om het gedurende de eerste weken van het bestaan van een agent achter een goedkeuring te plaatsen zodat je slechte waarschuwingen kunt opmerken voordat ze worden verzonden, maar de meeste operators halen de goedkeuring weg zodra de agent betrouwbare output produceert.

### See also

- [Ban user](#tool-ban-user) - the next step up in escalation.
- [Agent Memory System](#agent-memory-system) - where warning records live.