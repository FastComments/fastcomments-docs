Fügen Sie diese Zeile zur Gemfile Ihrer Anwendung hinzu:

```ruby
gem 'fastcomments'
```

Und dann ausführen:

```bash
bundle install
```

Oder installieren Sie es selbst als:

```bash
gem install fastcomments
```

### Bibliotheksinhalte

Diese Bibliothek enthält den generierten API-Client und die SSO-Dienstprogramme, um die Arbeit mit der API zu erleichtern.

- [Dokumentation der API-Client-Bibliothek](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Öffentliche vs. gesicherte APIs

Für den API-Client gibt es zwei Klassen, `DefaultApi` und `PublicApi`. Die `DefaultApi` enthält Methoden, die Ihren API-Schlüssel erfordern, und `PublicApi` enthält API-Aufrufe, die direkt von einem Browser/Mobilgerät/etc. ohne Authentifizierung ausgeführt werden können.