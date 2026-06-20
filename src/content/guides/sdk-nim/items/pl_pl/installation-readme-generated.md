### Korzystanie z Nimble

```bash
nimble install fastcomments
```

### Budowanie ze źródeł

```bash
nimble build
```

### Zawartość biblioteki

Ta biblioteka zawiera wygenerowanego klienta API oraz narzędzia SSO, aby ułatwić pracę z API.

- [Dokumentacja biblioteki klienta API](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### API publiczne vs zabezpieczone

W kliencie API znajdują się trzy moduły API: `api_default`, `api_public` oraz `api_moderation`. `api_default` zawiera metody, które wymagają Twojego klucza API, a `api_public` zawiera wywołania API, które można wykonywać bezpośrednio z przeglądarki/urządzenia mobilnego itp. bez uwierzytelniania. Moduł `api_moderation` zawiera metody dla panelu moderatora.

Metody `api_moderation` obejmują listowanie, zliczanie, wyszukiwanie i eksportowanie komentarzy oraz ich logów; akcje moderacyjne takie jak usuwanie/przywracanie komentarzy, zgłaszanie, ustawianie statusu do przeglądu/spamu/akceptacji, dostosowywanie głosów oraz ponowne otwieranie/zamykanie wątków; bany (zablokowanie użytkownika od komentowania, cofanie bana, podsumowania przed banem, status i preferencje bana oraz liczba zbanowanych użytkowników); oraz odznaki i zaufanie (przyznawanie/usuwanie odznaki, listowanie odznak ręcznych, pobieranie/ustawianie współczynnika zaufania użytkownika oraz pobieranie wewnętrznego profilu użytkownika). Każda metoda `api_moderation` przyjmuje parametr `sso`, dzięki czemu wywołanie jest uwierzytelnione jako moderator SSO.