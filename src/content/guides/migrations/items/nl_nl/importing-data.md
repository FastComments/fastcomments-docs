Hoewel FastComments Support kan helpen met migraties, kunnen de meeste eenvoudig worden uitgevoerd en gevolgd zonder tussenkomst
van medewerkers van de support.

We ondersteunen standaard het importeren van exports van de volgende providers:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (via de plugin, of een XML- of CSV-export)
- AnyComment (via WordPress Import/Export)

Door [hier](https://fastcomments.com/auth/my-account/manage-data/import) te navigeren kunnen we het bestand met de te migreren gegevens uploaden.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; alt='FastComments importpagina met de providerkeuze en de velden voor het uploaden van een exportbestand'; title='Het importpaginaformulier' app-screenshot-end]

### Imports volgen

FastComments gebruikt een taakverwerkingssysteem voor het verwerken van imports en exports. Zodra het systeem je taak heeft opgepakt, zal het
periodiek de status van de taak rapporteren in de import- of export-UI.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; alt='Importpagina met een lopende importtaak en de status die door het taakverwerkingssysteem wordt gerapporteerd'; title='Status van importtaak' app-screenshot-end]

Merk op dat de status van Imports en Exports door alle beheerders in het account kan worden bekeken.

Als je taak mislukt, wordt deze niet automatisch opnieuw gestart. De import moet opnieuw worden uitgevoerd. Als een import of export mislukt,
worden onze systeembeheerders automatisch op de hoogte gebracht. Als we een probleem identificeren, nemen we contact met je op om te zien of we kunnen helpen.

### De import opnieuw uitvoeren

Bij sommige migraties is het nodig om de import meerdere keren uit te voeren. Het is bijvoorbeeld gebruikelijk om eerst een testmigratie
uit te voeren en daarna de import opnieuw uit te voeren met de meest recente gegevens voordat de schakelaar wordt omgezet.

Het opnieuw importeren van dezelfde inhoud **zal geen duplicaten creëren**.

### Gegevensbeveiliging en verloop

Importbestanden zijn op geen enkele manier toegankelijk via externe verzoeken, en importbestanden worden zo snel mogelijk
verwijderd uit ons systeem zodra de import is voltooid.