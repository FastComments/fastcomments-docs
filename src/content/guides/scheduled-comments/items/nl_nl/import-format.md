### Formaatvoorbeeld

De Geplande Reacties CSV ziet er zo uit:

|ID |Parent ID|URL ID       |URL|Name         |Avatar|Comment                              | Hours | Minutes | Seconds |
|---|---------|-------------|---|-------------|------|-------------------------------------|-------|---------|---------|
|1  |         |scheduledtest|   |Test Person 1|      |Hello!                               | 0     | 0       | 3       |
|2  |1        |scheduledtest|   |Test Person 2|      |How are you?                         | 0     | 0       | 10      |
|3  |2        |scheduledtest|   |Test Person 3|      |Leave them alone                     | 0     | 1       | 30      |
|4  |         |scheduledtest|   |Test Person 4|      |Does anyone have a cute cat to share?| 1     | 10      | 0       |


### Formaatdetails

Het Geplande Reacties CSV-bestand ondersteunt de volgende kolommen:

- ID
- Parent ID
- URL ID
- URL
- Name
- Avatar
- Comment
- Hours
- Minutes
- Seconds

De volgende kolommen zijn **verplicht**:

- ID
- URL ID
- Name
- Comment
- Hours
- Minutes
- Seconds

Je hebt de Parent ID-kolom nodig als je geautomatiseerde geneste antwoorden wilt ondersteunen.

### Hoe het formaat werkt

Het importformaat werkt als volgt:

1. Je definieert een rij in de CSV voor elke reactie die je wilt plaatsen.
2. De reactie wordt geplaatst na de opgegeven vertraging (uren + minuten + seconden).
   1. Voor handmatig geplande imports zijn vertragingen relatief aan wanneer je op "play" drukt in de UI, nadat de import is voltooid - **niet wanneer de import start**.
   2. Voor automatisch geplande imports is de vertraging vanaf het moment van paginalading.
3. Je moet een ID definieren. Je kunt gewoon oplopende identifiers gebruiken zoals 1, 2, 3, 4, 5.
4. Als je geneste antwoorden wilt gebruiken, stel je gewoon de `Parent ID` kolomwaarde in op de `ID` van een andere reactie.
5. Het `Comment`-veld ondersteunt dezelfde syntaxis die FastComments ondersteunt in zijn reactiewidget voor het stylen van tekst en toevoegen van afbeeldingen.
6. Het `Avatar`-veld moet, indien gebruikt, een openbaar toegankelijke afbeelding zijn. Het wordt gekopieerd naar en geserveerd vanaf onze CDN.
7. Je kunt alle reacties verwijderen na het afspelen, of als het afspelen wordt gestopt.
8. Verwijdering gebeurt live, dus je kunt dezelfde geplande import steeds opnieuw gebruiken.

### Een voorbeeld

[Een voorbeeld CSV-bestand is hier](/csv/fastcomments-scheduled-comments-example.csv).
