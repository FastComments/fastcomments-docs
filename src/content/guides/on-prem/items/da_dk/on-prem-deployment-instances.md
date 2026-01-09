### Nødvendige komponenter

For On-Prem består FastComments kun af en applikationsserver og en database. Vi har forenklet udrulningen, så applikationen kan håndtere al trafik direkte uden at tilføje andre komponenter.

Applikationsserveren leveres i et Docker-image og kan udrulles med enhver container management-løsning.

Databasen, MongoDB, kan køres selv eller hostes af en anden udbyder som AWS DocumentDB eller MongoDB Atlas.

FastComments er i øjeblikket testet med MongoDB 7, men vi sigter efter at være DocumentDB-kompatible for at lette udrulningen.

### Instansstørrelser

Du vil opleve, at FastComments er ret veloptimeret og ikke kræver store maskiner for selve applikationen for at holde lave P99s.

Alle batch- og cron jobs bruger streaming for at begrænse det samlede hukommelsesforbrug.

Tabellerne nedenfor for applikationsserveren og databasen kan hjælpe med dimensionering.

### Applikationsserverinstanser


| Concurrent Users | Total Cluster CPUs | Total Cluster Memory |
|------------------|--------------------|----------------------|
| 100              | 1                  | 256mb                |
| 1K               | 2                  | 512mb                |
| 10K              | 8                  | 1gb                  |
| 100K             | 32                 | 8gb                  |
| 1M               | 64                 | 64gb                 |

For eksempel bruger en enkelt kerne, der håndterer omkring 100 kommentarsamtaler i sekundet, normalt aldrig mere end 250mb RSS.

### Database-serverinstanser

Dimensionering af databasen afhænger af working set-størrelsen, hvilket er mængden af data, du tilgår på et givent tidspunkt, samt samtidige forespørgsler.

FastComments er ret venlig mod Mongo, idet den for de varme forespørgsler bruger index hints, streaming cursors og har concurrency-begrænsninger på forskellige områder for at forhindre overbelastning af downstream-systemer.

Nedenstående er en generel retningslinje for database-instansstørrelser. **Bemærk, at dette er __pr. instans__, ikke de samlede ressourcer i clustret**.

| Concurrent Users | Comments Stored | CPUs Per Instance | Memory Per Instance |
|------------------|-----------------|-------------------|---------------------|
| 100              | 1k              | 1                 | 256mb               |
| 1K               | 5k              | 2                 | 512mb               |
| 10K              | 100k            | 8                 | 2gb                 |
| 100K             | 500k            | 16                | 8gb                 |
| 1M               | 5M              | 32                | 32gb                |

Ovenstående tabeller er konservative estimater. Du kan opleve, at de faktiske krav varierer baseret på din specifikke konfiguration (side-størrelser, kommentarmængde osv).