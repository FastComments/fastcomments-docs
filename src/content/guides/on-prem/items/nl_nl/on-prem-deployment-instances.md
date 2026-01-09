### Vereiste componenten

Voor On-Prem bestaat FastComments alleen uit een applicatieserver en een database. We hebben de deployment vereenvoudigd zodat
de applicatie al het verkeer direct kan afhandelen zonder extra componenten toe te voegen.

De applicatieserver wordt geleverd in een Docker image en kan worden ingezet met elke containerbeheeroplossing.

De database, MongoDB, kan zelf worden beheerd of gehost worden door een andere provider zoals AWS DocumentDB of MongoDB Atlas.

FastComments is momenteel getest met MongoDB 7, maar we streven ernaar DocumentDB-compatibel te zijn om de deployment te vereenvoudigen.

### Instancegroottes

Je zult merken dat FastComments redelijk goed geoptimaliseerd is en geen grote machines voor de applicatie zelf vereist om lage P99s te behouden.

Alle batch- en cron-jobs gebruiken streaming om het totale geheugengebruik te beperken.

De onderstaande tabellen voor de applicatieserver en database kunnen helpen bij het bepalen van de juiste grootte.

### Applicatieserverinstanties


| Concurrent Users | Total Cluster CPUs | Total Cluster Memory |
|------------------|--------------------|----------------------|
| 100              | 1                  | 256mb                |
| 1K               | 2                  | 512mb                |
| 10K              | 8                  | 1gb                  |
| 100K             | 32                 | 8gb                  |
| 1M               | 64                 | 64gb                 |

Bijvoorbeeld gebruikt een enkele core die ongeveer 100 reactiedraden per seconde bedient gewoonlijk nooit meer dan 250mb RSS.

### Databaseserverinstanties

Het dimensioneren van de database hangt af van de grootte van de werkset, dat is de hoeveelheid data die je op een gegeven moment benadert, evenals gelijktijdige verzoeken.

FastComments is redelijk vriendelijk voor Mongo, in die zin dat voor de hot queries het gebruikmaakt van index hints, streaming cursors, en er in verschillende gebieden concurrency limits zijn
om overbelasting van downstream-systemen te voorkomen.

Hieronder staat een algemene richtlijn voor database-instantiegroottes. **Let op dat dit __per instantie__ is, niet de totale resources in het cluster**.

| Concurrent Users | Comments Stored | CPUs Per Instance | Memory Per Instance |
|------------------|-----------------|-------------------|---------------------|
| 100              | 1k              | 1                 | 256mb               |
| 1K               | 5k              | 2                 | 512mb               |
| 10K              | 100k            | 8                 | 2gb                 |
| 100K             | 500k            | 16                | 8gb                 |
| 1M               | 5M              | 32                | 32gb                |

De bovenstaande tabellen zijn conservatieve schattingen. Je kunt merken dat de daadwerkelijke vereisten verschillen op basis van je specifieke configuratie (paginagroottes, commentaargrootte, enz.).