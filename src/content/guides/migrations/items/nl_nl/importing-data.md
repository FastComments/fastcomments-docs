---
Hoewel FastComments Support kan helpen bij migraties, kunnen de meeste eenvoudig worden uitgevoerd en gemonitord zonder tussenkomst van supportmedewerkers.

We ondersteunen native het importeren van exports van de volgende providers:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (via de plugin, of een XML- of CSV-export)
- AnyComment (Via WordPress Import/Export)

Door naar [hier](https://fastcomments.com/auth/my-account/manage-data/import) te navigeren, kunnen we het bestand met de te migreren gegevens uploaden.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; alt='FastComments importpagina met de providerselectie en bestandsuploadvelden voor een exportbestand'; title='Het importpagina-formulier' app-screenshot-end]

### Monitoring Imports

FastComments maakt gebruik van een taakverwerkingssysteem voor het verwerken van imports en exports. Zodra het systeem uw taak heeft opgepikt, zal het periodiek de status van de taak rapporteren in de import- of export-UI.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; alt='Importpagina die een lopende importtaak toont en de status gerapporteerd door het taakverwerkingssysteem'; title='Importtaakstatus' app-screenshot-end]

Let op dat de status voor imports en exports zichtbaar is voor alle beheerders in het account.

Als uw taak faalt, wordt deze niet automatisch opnieuw gestart. De import moet opnieuw worden geprobeerd. Als een import of export faalt, worden onze systeembeheerders automatisch op de hoogte gebracht. Als we een probleem identificeren, nemen we contact met u op om te zien of we kunnen helpen.

### De import opnieuw uitvoeren

Tijdens sommige migraties is het nodig om de import meerdere keren uit te voeren. Bijvoorbeeld, het is gebruikelijk om een eerste migratiepas voor testen uit te voeren, en vervolgens de import opnieuw te draaien met de nieuwste gegevens voordat de omschakeling plaatsvindt.

Het opnieuw importeren van dezelfde inhoud **zal geen duplicaten creëren**.

### Gegevensbeveiliging en vervaldatum

Importbestanden zijn op geen enkele manier toegankelijk via externe verzoeken, en importbestanden worden uit ons systeem verwijderd zodra de import is voltooid.

---