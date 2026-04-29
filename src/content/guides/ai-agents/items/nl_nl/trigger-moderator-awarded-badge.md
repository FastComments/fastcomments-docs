---
Wordt geactiveerd wanneer een moderator een badge toekent aan een gebruiker.

### Context die de agent ontvangt

- De **badge ID** van de toegekende badge.
- De **triggering user ID** - de moderator die de badge heeft toegekend.
- Optionele thread / gebruikersgeschiedenis / paginacontext zoals geconfigureerd.

De fire-site bevat **geen** een `commentId` in de trigger-payload, zelfs niet als de badge werd toegekend met betrekking tot een specifieke reactie.

### Wie activeert dit

Een handeling door een menselijke moderator.

### Opmerkelijk

- Alleen de **badge ID** wordt opgenomen; de agent ontvangt niet de badgemetadata (naam, afbeelding). Als de agent moet redeneren over *welke* badge is toegekend, voeg die context toe in de [initiële prompt](#personality-prompt) of de [gemeenschapsrichtlijnen](#community-guidelines).
- De trigger wordt geactiveerd eenmaal per badge-toekenning, niet per gebruiker. Het twee keer toekennen van dezelfde badge aan een gebruiker activeert de trigger twee keer (elke toekenning is een afzonderlijk evenement).

### Veelvoorkomende toepassingen

- **Wederzijdse erkenning** - een agent kan een antwoord plaatsen met "dank voor de geweldige bijdrage" wanneer een specifieke badge wordt toegekend.
- **Externe erkenningsworkflow** via [Webhooks](#webhooks-overview) - spiegel badge-toekenningen naar je eigen systeem voor gebruikersbetrokkenheid.
- **Geheugenregistratie** - aantekeningen zoals "deze gebruiker is een erkende bijdrager" zodat toekomstige moderatieagenten dit kunnen meenemen in hun beslissingen.

---