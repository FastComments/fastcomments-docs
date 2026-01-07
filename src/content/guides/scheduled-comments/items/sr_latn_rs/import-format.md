### Primer formata

CSV zakazanih komentara izgleda ovako:

|ID |Parent ID|URL ID       |URL|Name         |Avatar|Comment                              | Hours | Minutes | Seconds |
|---|---------|-------------|---|-------------|------|-------------------------------------|-------|---------|---------|
|1  |         |scheduledtest|   |Test Person 1|      |Hello!                               | 0     | 0       | 3       |
|2  |1        |scheduledtest|   |Test Person 2|      |How are you?                         | 0     | 0       | 10      |
|3  |2        |scheduledtest|   |Test Person 3|      |Leave them alone                     | 0     | 1       | 30      |
|4  |         |scheduledtest|   |Test Person 4|      |Does anyone have a cute cat to share?| 1     | 10      | 0       |


### Detalji formata

CSV datoteka zakazanih komentara podrzava sledece kolone:

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

Sledece kolone su **obavezne**:

- ID
- URL ID
- Name
- Comment
- Hours
- Minutes
- Seconds

Trebace vam kolona Parent ID ako zelite da podrziite automatizovane ugnezdjene odgovore.

### Kako format funkcionise

Format uvoza funkcionise ovako:

1. Definisite red u CSV-u za svaki komentar koji zelite da objavite.
2. Komentar se objavljuje nakon navedenog kasnjenja (sati + minuti + sekunde).
   1. Za rucno zakazane uvoze, kasnjenja su relativna prema trenutku kada pritisnete "play" u korisnickom interfejsu, nakon sto je uvoz zavrsen - **ne kada uvoz pocinje**.
   2. Za automatski zakazane uvoze, kasnjenje je od trenutka ucitavanja stranice.
3. Morate definisati ID. Mozete jednostavno koristiti rastuce identifikatore poput 1, 2, 3, 4, 5.
4. Ako zelite da koristite ugnezdjene odgovore, jednostavno postavite vrednost kolone `Parent ID` na `ID` drugog komentara.
5. Polje `Comment` podrzava istu sintaksu koju FastComments podrzava u svom vidzetu za komentare za stilizovanje teksta i dodavanje slika.
6. Polje `Avatar`, ako se koristi, mora biti javno dostupna slika. Bice kopirana i sluzena sa naseg CDN-a.
7. Mozete izbrisati sve komentare nakon reprodukcije, ili ako se reprodukcija zaustavi.
8. Brisanje se desava uzivo, tako da mozete ponovo koristiti isti zakazani uvoz iznova i iznova.

### Primer

[Primer CSV datoteke je ovde](/csv/fastcomments-scheduled-comments-example.csv).
