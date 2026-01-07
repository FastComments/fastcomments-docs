### Primer formata

CSV nacrtovanih komentarjev izgleda tako:

|ID |Parent ID|URL ID       |URL|Name         |Avatar|Comment                              | Hours | Minutes | Seconds |
|---|---------|-------------|---|-------------|------|-------------------------------------|-------|---------|---------|
|1  |         |scheduledtest|   |Test Person 1|      |Hello!                               | 0     | 0       | 3       |
|2  |1        |scheduledtest|   |Test Person 2|      |How are you?                         | 0     | 0       | 10      |
|3  |2        |scheduledtest|   |Test Person 3|      |Leave them alone                     | 0     | 1       | 30      |
|4  |         |scheduledtest|   |Test Person 4|      |Does anyone have a cute cat to share?| 1     | 10      | 0       |


### Podrobnosti formata

CSV datoteka nacrtovanih komentarjev podpira naslednje stolpce:

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

Naslednji stolpci so **obvezni**:

- ID
- URL ID
- Name
- Comment
- Hours
- Minutes
- Seconds

Potrebovali boste stolpec Parent ID, ce zelite podpirati avtomatizirane gnezdene odgovore.

### Kako format deluje

Format uvoza deluje tako:

1. Dolocite vrstico v CSV za vsak komentar, ki ga zelite objaviti.
2. Komentar je objavljen po navedeni zamudi (ure + minute + sekunde).
   1. Za rocno nacrtovane uvoze so zamude relativne glede na cas, ko pritisnete "play" v uporabniskem vmesniku, potem ko je uvoz koncen - **ne ko se uvoz zacne**.
   2. Za samodejno nacrtovane uvoze je zamuda od casa nalaganja strani.
3. Morate dolociti ID. Lahko preprosto uporabite narascejoce identifikatorje kot 1, 2, 3, 4, 5.
4. Ce zelite uporabiti gnezdene odgovore, preprosto nastavite vrednost stolpca `Parent ID` na `ID` drugega komentarja.
5. Polje `Comment` podpira enako sintakso, ki jo FastComments podpira v svojem pripomocku za komentarje za oblikovanje besedila in dodajanje slik.
6. Polje `Avatar`, ce se uporablja, mora biti javno dostopna slika. Kopirana bo in servirana z nasega CDN.
7. Vse komentarje lahko izbrisete po predvajanju ali ce se predvajanje ustavi.
8. Brisanje se zgodi v zivo, tako da lahko isto nacrtovano uvozeno znova in znova uporabite.

### Primer

[Primer CSV datoteke je tukaj](/csv/fastcomments-scheduled-comments-example.csv).
