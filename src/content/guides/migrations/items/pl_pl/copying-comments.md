W przypadku konieczności przenoszenia danych, FastComments udostępnia narzędzie samoobsługowe do przenoszenia komentarzy między stronami i artykułami.

Oto jak wygląda formularz kopiowania komentarzy:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; alt='Formularz kopiowania komentarzy z polem From URL ID oraz polami To URL ID i URL'; title='Formularz kopiowania komentarzy' app-screenshot-end]

### Wypełnianie pól "From"

Aby zdecydować, skąd przenieść komentarze, potrzebujemy po prostu znać źródłowy `URL ID`.

Jeśli nie przekazujesz wartości `urlId` w konfiguracji widgetu komentarza, będzie to „czysta” wersja adresu URL strony.

Możesz zobaczyć, jakie wartości mają Twoje komentarze w polu `URL ID`, eksportując je.

### Wypełnianie pól "To"

Aby zdecydować, dokąd przenieść komentarze, musimy znać docelowy `URL ID` oraz `URL`.

`URL ID` będzie koszykiem, w którym znajdzie się komentarz. Pole `URL` służy do tego, abyś mógł/mogła przejść bezpośrednio do komentarza z e‑maili i narzędzi moderacji.

#### WordPress

Jeśli używasz WordPressa, w narzędziu migracji wprowadziłbyś na przykład identyfikatory artykułów w pola To/From `URL ID`, zamiast adresu URL.