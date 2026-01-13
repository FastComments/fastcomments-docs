W przypadku, gdy konieczne będzie przeniesienie danych, FastComments udostępnia narzędzie samoobsługowe do przenoszenia komentarzy
między stronami i artykułami.

Tak wygląda formularz kopiowania komentarzy:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### Wypełnianie pól "From"

Aby zdecydować, skąd przenieść komentarze, wystarczy znać źródłowe `URL ID`.

Jeśli nie przekazujesz wartości dla `urlId` w konfiguracji widżetu komentarzy, będzie to "czysta" wersja URL strony.

Możesz zobaczyć, jakie wartości mają Twoje komentarze dla `URL ID` eksportując je.

### Wypełnianie pól "To"

Aby zdecydować, dokąd przenieść komentarze, musimy znać docelowe `URL ID` oraz `URL`.

`URL ID` będzie kontenerem, do którego przypisany jest komentarz. Pole `URL` jest używane tak, abyś mógł/mogła przejść bezpośrednio
do komentarza z wiadomości e-mail i narzędzi moderacji.

#### WordPress

Jeśli używasz WordPressa, na przykład w polach To/From `URL ID` w narzędziu migracji wpiszesz identyfikatory artykułów,
zamiast URL.