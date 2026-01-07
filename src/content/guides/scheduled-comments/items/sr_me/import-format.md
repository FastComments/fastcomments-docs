### Primjer formata

CSV zakazanih komentara izgleda ovako:

|ID |Parent ID|URL ID       |URL|Name         |Avatar|Comment                              | Hours | Minutes | Seconds |
|---|---------|-------------|---|-------------|------|-------------------------------------|-------|---------|---------|
|1  |         |scheduledtest|   |Test Person 1|      |Hello!                               | 0     | 0       | 3       |
|2  |1        |scheduledtest|   |Test Person 2|      |How are you?                         | 0     | 0       | 10      |
|3  |2        |scheduledtest|   |Test Person 3|      |Leave them alone                     | 0     | 1       | 30      |
|4  |         |scheduledtest|   |Test Person 4|      |Does anyone have a cute cat to share?| 1     | 10      | 0       |


### Detalji formata

CSV datoteka zakazanih komentara podrzava sljedece kolone:

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

Sljedece kolone su **obavezne**:

- ID
- URL ID
- Name
- Comment
- Hours
- Minutes
- Seconds

Trebat ce vam kolona Parent ID ako zelite podrzati automatizirane ugniježdene odgovore.

### Kako format funkcionira

Format uvoza funkcionira ovako:

1. Definirate red u CSV-u za svaki komentar koji zelite objaviti.
2. Komentar se objavljuje nakon navedenog kasnjenja (sati + minute + sekunde).
   1. Za rucno zakazane uvoze, kasnjenja su relativna prema trenutku kada pritisnete "play" u korisnickom sucelju, nakon sto je uvoz zavrsen - **ne kada uvoz pocinje**.
   2. Za automatski zakazane uvoze, kasnjenje je od trenutka ucitavanja stranice.
3. Morate definirati ID. Mozete jednostavno koristiti rastuce identifikatore poput 1, 2, 3, 4, 5.
4. Ako zelite koristiti ugniježdene odgovore, jednostavno postavite vrijednost kolone `Parent ID` na `ID` drugog komentara.
5. Polje `Comment` podrzava istu sintaksu koju FastComments podrzava u svom widgetu za komentare za stiliziranje teksta i dodavanje slika.
6. Polje `Avatar`, ako se koristi, mora biti javno dostupna slika. Bit ce kopirana i posluzena sa naseg CDN-a.
7. Mozete izbrisati sve komentare nakon reprodukcije, ili ako se reprodukcija zaustavi.
8. Brisanje se dogada uzivo, tako da mozete ponovno koristiti isti zakazani uvoz iznova i iznova.

### Primjer

[Primjer CSV datoteke je ovdje](/csv/fastcomments-scheduled-comments-example.csv).
