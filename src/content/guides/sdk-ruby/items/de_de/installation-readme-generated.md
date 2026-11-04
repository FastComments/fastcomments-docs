Fügen Sie diese Zeile zur Gemfile Ihrer Anwendung hinzu:

```ruby
gem 'fastcomments'
```

Und führen Sie dann aus:

```bash
bundle install
```

Oder installieren Sie es selbst mit:

```bash
gem install fastcomments
```

### Bibliotheksinhalte

Diese Bibliothek enthält den generierten API-Client und die SSO-Dienstprogramme, um die Arbeit mit der API zu erleichtern.

- [API-Client-Bibliothek Dokumentation](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Öffentliche vs. gesicherte APIs

Für den API-Client gibt es drei Klassen: `DefaultApi`, `PublicApi` und `ModerationApi`. Die `DefaultApi` enthält Methoden, die Ihren API-Schlüssel erfordern, und die `PublicApi` enthält API-Aufrufe,
die direkt von einem Browser/Mobilgerät/usw. ohne Authentifizierung durchgeführt werden können. Die `ModerationApi` enthält die Methoden, die das Moderator-Dashboard antreiben.

Die `ModerationApi` bietet eine umfangreiche Suite von Live- und schnellen Moderations-APIs. Jede `ModerationApi`-Methode akzeptiert einen `sso`-Parameter und kann sich über SSO oder ein FastComments.com-Session-Cookie authentifizieren.