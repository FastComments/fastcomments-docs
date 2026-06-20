Fügen Sie diese Zeile der Gemfile Ihrer Anwendung hinzu:

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

- [Dokumentation der API-Client-Bibliothek](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Öffentliche vs. gesicherte APIs

Für den API-Client gibt es drei Klassen, `DefaultApi`, `PublicApi` und `ModerationApi`. Die `DefaultApi` enthält Methoden, die Ihren API-Schlüssel erfordern, und `PublicApi` enthält API-Aufrufe, die direkt aus einem Browser, einem Mobilgerät usw. ohne Authentifizierung ausgeführt werden können. Die `ModerationApi` enthält die Methoden, die das Moderator-Dashboard antreiben.

Die `ModerationApi` umfasst die Kommentarmoderation (list, count, search, logs, export), Moderationsaktionen (remove/restore, flag, set review/spam/approval status, votes, reopen/close thread), Sperren (ban from a comment, undo, pre-ban summaries, ban status/preferences, banned-user counts) sowie Abzeichen & Vertrauen (award/remove badge, manual badges, get/set trust factor, user internal profile). Jede `ModerationApi`-Methode akzeptiert einen `sso`-Parameter, damit die Anfrage im Namen eines per SSO authentifizierten Moderators gestellt werden kann.