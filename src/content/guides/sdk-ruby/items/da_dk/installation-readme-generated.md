Tilføj denne linje til din applikations Gemfile:

```ruby
gem 'fastcomments'
```

Og derefter udfør:

```bash
bundle install
```

Eller installer det manuelt som:

```bash
gem install fastcomments
```

### Biblioteksindhold

Dette bibliotek indeholder den genererede API-klient og SSO-værktøjerne for at gøre arbejdet med API'en lettere.

- [Dokumentation for API-klientbiblioteket](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Offentlige vs sikrede API'er

For API-klienten er der tre klasser, `DefaultApi`, `PublicApi` og `ModerationApi`. `DefaultApi` indeholder metoder, der kræver din API-nøgle, og `PublicApi` indeholder API-kald, som kan foretages direkte fra en browser/mobile enhed osv. uden godkendelse. `ModerationApi` indeholder metoderne, der driver moderatorpanelet.

`ModerationApi` dækker kommentar-moderation (liste, optælling, søgning, logs, eksport), moderationshandlinger (fjern/gendan, flag, sæt review/spam/godkendelsesstatus, stemmer, genåbn/luk tråd), bans (udban fra en kommentar, fortryd, forudgående ban-resuméer, ban-status/præferencer, antal bandlyste brugere) og badges & trust (tildel/fjern badge, manuelle badges, få/sæt tillidsfaktor, brugerens interne profil). Hver `ModerationApi`-metode accepterer en `sso`-parameter, så anmodningen kan foretages på vegne af en SSO-godkendt moderator.