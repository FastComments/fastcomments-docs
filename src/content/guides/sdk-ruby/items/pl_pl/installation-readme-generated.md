---
Dodaj tę linię do pliku Gemfile Twojej aplikacji:

```ruby
gem 'fastcomments'
```

A następnie wykonaj:

```bash
bundle install
```

Lub zainstaluj ją samodzielnie jako:

```bash
gem install fastcomments
```

### Zawartość biblioteki

Ta biblioteka zawiera wygenerowanego klienta API oraz narzędzia SSO, które ułatwiają pracę z API.

- [Dokumentacja biblioteki klienta API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Publiczne vs zabezpieczone API

Dla klienta API istnieją trzy klasy, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` zawiera metody wymagające klucza API, a `PublicApi` zawiera wywołania API, które mogą być wykonywane bezpośrednio z przeglądarki/urządzenia mobilnego itp. bez uwierzytelnienia. `ModerationApi` zawiera metody napędzające panel moderatora.

`ModerationApi` oferuje rozbudowany zestaw szybkich i bieżących API moderacji. Każda metoda `ModerationApi` przyjmuje parametr `sso` i może uwierzytelnić się za pomocą SSO lub ciasteczka sesji FastComments.com.