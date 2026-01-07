### Formateksempel

Den planlagte kommentar CSV ser saledes ud:

|ID |Parent ID|URL ID       |URL|Name         |Avatar|Comment                              | Hours | Minutes | Seconds |
|---|---------|-------------|---|-------------|------|-------------------------------------|-------|---------|---------|
|1  |         |scheduledtest|   |Test Person 1|      |Hello!                               | 0     | 0       | 3       |
|2  |1        |scheduledtest|   |Test Person 2|      |How are you?                         | 0     | 0       | 10      |
|3  |2        |scheduledtest|   |Test Person 3|      |Leave them alone                     | 0     | 1       | 30      |
|4  |         |scheduledtest|   |Test Person 4|      |Does anyone have a cute cat to share?| 1     | 10      | 0       |


### Formatdetaljer

Den planlagte kommentar CSV-fil understotter folgende kolonner:

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

Folgende kolonner er **pakravede**:

- ID
- URL ID
- Name
- Comment
- Hours
- Minutes
- Seconds

Du skal bruge Parent ID-kolonnen, hvis du vil understotte automatiserede indlejrede svar.

### Hvordan formatet virker

Importformatet fungerer saledes:

1. Du definerer en rakke i CSV'en for hver kommentar, du vil have postet.
2. Kommentaren postes efter den angivne forsinkelse (timer + minutter + sekunder).
   1. For manuelt planlagte importer er forsinkelser relative til, nar du trykker "play" i brugerfladen, efter importen er faerdig - **ikke nar importen starter**.
   2. For automatisk planlagte importer er forsinkelsen fra tidspunktet for sideindlasning.
3. Du skal definere et ID. Du kan bare bruge stigende identifikatorer som 1, 2, 3, 4, 5.
4. Hvis du vil bruge indlejrede svar, satter du bare `Parent ID` kolonnevaerdien til `ID` for en anden kommentar.
5. `Comment`-feltet understotter samme syntaks, som FastComments understotter i sin kommentarwidget til styling af tekst og tilfoejelse af billeder.
6. `Avatar`-feltet skal, hvis det bruges, vare et offentligt tilgangeligt billede. Det vil blive kopieret til og serveret fra vores CDN.
7. Du kan slette alle kommentarer efter afspilningen, eller hvis afspilningen stoppes.
8. Sletning sker live, sa du kan genbruge den samme planlagte import igen og igen.

### Et eksempel

[En eksempel CSV-fil er her](/csv/fastcomments-scheduled-comments-example.csv).
