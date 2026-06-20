Dodaj tę linię do pliku Gemfile swojej aplikacji:

```ruby
gem 'fastcomments'
```

A następnie wykonaj:

```bash
bundle install
```

Lub zainstaluj samodzielnie za pomocą:

```bash
gem install fastcomments
```

### Zawartość biblioteki

Ta biblioteka zawiera wygenerowanego klienta API oraz narzędzia SSO, które ułatwiają pracę z API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Publiczne a zabezpieczone API

Dla klienta API istnieją trzy klasy: `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` zawiera metody, które wymagają Twojego klucza API, a `PublicApi` zawiera wywołania API, które można wykonywać bezpośrednio z przeglądarki/urządzenia mobilnego itp. bez uwierzytelnienia. `ModerationApi` zawiera metody obsługujące panel moderatora.

`ModerationApi` obejmuje moderację komentarzy (lista, zliczanie, wyszukiwanie, logi, eksport), działania moderacyjne (usuwanie/przywracanie, oznaczanie, ustawianie statusu do przeglądu/spam/akceptacja, głosy, ponowne otwarcie/zamknięcie wątku), bany (zablokowanie z powodu komentarza, cofnięcie, podsumowania przed zablokowaniem, status/preferencje blokady, liczby zablokowanych użytkowników) oraz odznaki i zaufanie (przyznawanie/usuwanie odznaki, odznaki ręczne, pobieranie/ustawianie współczynnika zaufania, wewnętrzny profil użytkownika). Każda metoda `ModerationApi` akceptuje parametr `sso`, dzięki czemu żądanie może być wykonane w imieniu moderatora uwierzytelnionego przez SSO.