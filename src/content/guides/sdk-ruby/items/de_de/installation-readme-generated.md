Add this line to your application's Gemfile:

```ruby
gem 'fastcomments'
```

And then execute:

```bash
bundle install
```

Or install it yourself as:

```bash
gem install fastcomments
```

### Bibliotheksinhalt

Diese Bibliothek enthält den generierten API‑Client und die SSO‑Hilfsprogramme, um die Arbeit mit der API zu erleichtern.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Öffentliche vs gesicherte APIs

Für den API‑Client gibt es drei Klassen, `DefaultApi`, `PublicApi` und `ModerationApi`. Die `DefaultApi` enthält Methoden, die Ihren API‑Schlüssel benötigen, und `PublicApi` enthält API‑Aufrufe, die direkt von einem Browser/Mobilgerät/etc. ohne Authentifizierung durchgeführt werden können. Die `ModerationApi` enthält die Methoden, die das Moderator‑Dashboard betreiben.

Die `ModerationApi` bietet eine umfangreiche Suite von Live‑ und schnellen Moderations‑APIs. Jede `ModerationApi`‑Methode akzeptiert einen `sso`‑Parameter und kann sich über SSO oder ein FastComments.com‑Session‑Cookie authentifizieren.