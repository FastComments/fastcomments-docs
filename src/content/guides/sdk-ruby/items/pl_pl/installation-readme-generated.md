Dodaj tę linię do Gemfile swojej aplikacji:

```ruby
gem 'fastcomments'
```

A następnie wykonaj:

```bash
bundle install
```

Lub zainstaluj go samodzielnie:

```bash
gem install fastcomments
```

### Zawartość biblioteki

Ta biblioteka zawiera wygenerowanego klienta API oraz narzędzia SSO, które ułatwiają pracę z API.

- [Dokumentacja biblioteki klienta API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Publiczne vs Zabezpieczone API

Dla klienta API istnieją dwie klasy, `DefaultApi` i `PublicApi`. `DefaultApi` zawiera metody, które wymagają Twojego klucza API, a `PublicApi` zawiera wywołania API
które można wykonywać bezpośrednio z przeglądarki/urządzenia mobilnego itp. bez uwierzytelniania.
---