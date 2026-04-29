---
De Ban-tool is de meest ingrijpende actie die een agent kan uitvoeren. Het verbant een gebruiker uit je community, voor een vaste duur en met een paar opties.

### Wat het doet

De agent kiest een van zes duuropties:

- Één uur
- Één dag
- Één week
- Één maand
- Zes maanden
- Één jaar

Hij kiest ook tussen een **zichtbare ban** (de gebruiker ziet een duidelijk verbodbericht en kan in beroep gaan) en een **shadow ban** (de gebruiker kan blijven posten maar zijn inhoud wordt verborgen voor andere gebruikers). De instructies van het platform geven de agent de voorkeur voor zichtbare bans bij eerste of grensgevallen, en shadow bans voor duidelijk kwaadaardige terugkerende overtreders.

### De twee destructieve subopties

Twee extra opties zijn **standaard verborgen voor de agent**. Om een van beide in te schakelen, vink het overeenkomstige selectievakje aan in de **Ban options** sectie van het bewerkingsformulier van de agent:

- **Allow deleting all of the user's comments.** Wanneer ingeschakeld kan de agent er ook voor kiezen om alle reacties die de gebande gebruiker ooit in jouw tenant heeft geplaatst te verwijderen. Reserveer voor duidelijke spam, doxxing of gecoördineerd misbruik waarbij de bestaande inhoud geen waarde heeft. **Destructief en onomkeerbaar.**
- **Allow banning by IP.** Wanneer ingeschakeld kan de agent er ook voor kiezen om het IP-adres waarvan de reactie is geplaatst te bannen. Nuttig tegen alt-account ban-ontduiking. **Vermijd bij gedeelde IP-adressen** (bedrijf, school, mobiele providers) - onschuldige gebruikers op hetzelfde netwerk worden geblokkeerd.

Het platform dwingt deze ook server-side af: zelfs als de agent doorslaat en probeert de optie aan te roepen, wordt het verzoek geweigerd tenzij je hebt ingestemd.

### Escalatiebeleid

Voordat er geband wordt, geeft het platform de agent de opdracht om:

1. Het [agentgeheugen](#agent-memory-system) te doorzoeken op eerdere waarschuwingen of notities over de gebruiker.
2. Bij eerste overtredingen de voorkeur te geven aan het [waarschuwen](#tool-warn-user) van de gebruiker boven bannen.
3. De waarschuwingsstap alleen over te slaan bij duidelijk ernstige gevallen (illegale inhoud, doxxing, gecoördineerde spam) - en uit te leggen waarom in de rechtvaardiging.

Dit beleid staat in de instructies van de agent, niet als harde server-side regel, wat de reden is waarom het sterk wordt aanbevolen om **bans achter goedkeuring te plaatsen**.

### EU-regio: menselijke goedkeuring vereist

In de EU-regio is deze tool volgens Artikel 17 van de Digital Services Act **vergrendeld voor goedkeuring**. Elke ban door een agent op een tenant in de EU-regio belandt in de [approvals inbox](#approval-workflow) voor menselijke beoordeling. Zie [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Aanbevelingen

- Stel overal goedkeuring verplicht gedurende minstens de eerste maand.
- Zet de **delete-all-comments** optie altijd achter goedkeuring als je deze inschakelt - het is onomkeerbaar.
- Overweeg om de **IP ban** optie achter goedkeuring te plaatsen, ook nadat de agent vertrouwen heeft opgebouwd - de kosten van een IP-ban op een gedeeld netwerk verschijnen niet in de runhistorie van de agent.

### Zie ook

- [Banning Users](/guide-moderation.html#banning-users) en [Banning Users With Wildcards](/guide-moderation.html#banning-users-wildcards) in de moderatiegids voor hoe bans platformbreed werken.
- [Waarschuw gebruiker](#tool-warn-user) - de mildere escalatiestap.

---