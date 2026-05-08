Hoewel FastComments Support kan helpen bij migraties, kunnen de meeste eenvoudig worden uitgevoerd en gemonitord zonder tussenkomst van ondersteunend personeel.

We ondersteunen het importeren van exports van de volgende providers op native wijze:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via de plugin)
- AnyComment (via WordPress Import/Export)

Door [hier](https://fastcomments.com/auth/my-account/manage-data/import) naartoe te navigeren kunt u het bestand uploaden met de te migreren gegevens.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Monitoring Imports

FastComments gebruikt een jobverwerkingssysteem voor het verwerken van imports en exports. Zodra het systeem uw taak heeft opgepakt, rapporteert het periodiek de status van de taak in de import- of export-UI.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Houd er rekening mee dat de status voor imports en exports door alle beheerders in het account bekeken kan worden.

Als uw taak faalt, wordt deze niet automatisch opnieuw gestart. De import moet opnieuw worden geprobeerd. Als een import of export faalt, worden onze systeembeheerders automatisch op de hoogte gesteld. Als we een probleem identificeren, nemen we contact met u op om te kijken of we kunnen helpen.

### Re-Running The Import

Tijdens sommige migraties is het noodzakelijk om de import meerdere keren uit te voeren. Bijvoorbeeld, het is gebruikelijk om een eerste pass-migratie voor tests uit te voeren en vervolgens de import opnieuw te draaien met de nieuwste gegevens voordat u de schakelaar omzet.

Het opnieuw importeren van dezelfde inhoud **zal geen duplicaten aanmaken**.

### Data Security and Expiration

Importbestanden zijn op geen enkele manier toegankelijk via externe verzoeken en importbestanden worden uit ons systeem verwijderd zodra de import is voltooid.