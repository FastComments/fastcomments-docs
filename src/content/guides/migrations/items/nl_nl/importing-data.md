---
Hoewel FastComments Support kan helpen bij migraties, kunnen de meeste gemakkelijk worden uitgevoerd en bewaakt zonder enige tussenkomst
van ondersteuningsmedewerkers.

We ondersteunen het native importeren van exports van de volgende providers:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via de plugin)

Door naar [hier](https://fastcomments.com/auth/my-account/manage-data/import) te gaan kunnen we het bestand met de te migreren gegevens uploaden.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Importbewaking

FastComments gebruikt een jobverwerkingssysteem voor het verwerken van importen en exporten. Zodra het systeem uw taak heeft opgepikt, zal het
periodiek de status van de taak rapporteren in de import- of export-UI.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Houd er rekening mee dat de status voor importen en exporten zichtbaar is voor alle beheerders in het account.

Als uw taak faalt, wordt deze niet automatisch opnieuw gestart. De import zal opnieuw moeten worden geprobeerd. Als een import of export faalt,
worden onze systeembeheerders automatisch gewaarschuwd. Als we een probleem vaststellen, nemen we contact met u op om te kijken of we kunnen helpen.

### Import opnieuw uitvoeren

Tijdens sommige migraties is het noodzakelijk de import meerdere keren uit te voeren. Bijvoorbeeld is het gebruikelijk eerst een eerste migratiepass
uit te voeren voor tests, en daarna de import opnieuw uit te voeren met de meest recente gegevens voordat u live gaat.

Het opnieuw importeren van dezelfde inhoud **zal geen duplicaten aanmaken**.

### Gegevensbeveiliging en vervaldatum

Importbestanden zijn op geen enkele manier toegankelijk via externe aanvragen, en importbestanden worden uit ons systeem verwijderd zodra
de import is voltooid.

---