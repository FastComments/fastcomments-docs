### Przyklad formatu

CSV Zaplanowanych Komentarzy wyglada tak:

|ID |Parent ID|URL ID       |URL|Name         |Avatar|Comment                              | Hours | Minutes | Seconds |
|---|---------|-------------|---|-------------|------|-------------------------------------|-------|---------|---------|
|1  |         |scheduledtest|   |Test Person 1|      |Hello!                               | 0     | 0       | 3       |
|2  |1        |scheduledtest|   |Test Person 2|      |How are you?                         | 0     | 0       | 10      |
|3  |2        |scheduledtest|   |Test Person 3|      |Leave them alone                     | 0     | 1       | 30      |
|4  |         |scheduledtest|   |Test Person 4|      |Does anyone have a cute cat to share?| 1     | 10      | 0       |


### Szczegoly formatu

Plik CSV Zaplanowanych Komentarzy obsluguje nastepujace kolumny:

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

Nastepujace kolumny sa **wymagane**:

- ID
- URL ID
- Name
- Comment
- Hours
- Minutes
- Seconds

Bedziesz potrzebowal kolumny Parent ID, jesli chcesz obslugiwac zautomatyzowane zagniezddone odpowiedzi.

### Jak dziala format

Format importu dziala w nastepujacy sposob:

1. Definiujesz wiersz w CSV dla kazdego komentarza, ktory chcesz opublikowac.
2. Komentarz jest publikowany po okreslonym opoznieniu (godziny + minuty + sekundy).
   1. Dla recznie zaplanowanych importow, opoznienia sa wzgledem momentu nacisniecia "play" w interfejsie uzytkownika, po zakonczeniu importu - **nie kiedy import sie zaczyna**.
   2. Dla automatycznie zaplanowanych importow, opoznienie jest od momentu zaladowania strony.
3. Musisz zdefiniowac ID. Mozesz po prostu uzywac rosnacych identyfikatorow jak 1, 2, 3, 4, 5.
4. Jesli chcesz uzywac zagniezdonych odpowiedzi, po prostu ustaw wartosc kolumny `Parent ID` na `ID` innego komentarza.
5. Pole `Comment` obsluguje te sama skladnie, ktora FastComments obsluguje w swoim widgecie komentarzy do stylizowania tekstu i dodawania obrazow.
6. Pole `Avatar`, jesli jest uzywane, musi byc publicznie dostepnym obrazem. Zostanie skopiowane i serwowane z naszego CDN.
7. Mozesz usunac wszystkie komentarze po odtworzeniu, lub jesli odtwarzanie zostanie zatrzymane.
8. Usuwanie odbywa sie na zywo, wiec mozesz wielokrotnie uzywac tego samego zaplanowanego importu.

### Przyklad

[Przykladowy plik CSV jest tutaj](/csv/fastcomments-scheduled-comments-example.csv).
