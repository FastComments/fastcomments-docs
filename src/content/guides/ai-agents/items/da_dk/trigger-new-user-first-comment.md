Udløses, når en bruger poster sin første kommentar på dette site (din tenant). Dette er **én gang per bruger** - efterfølgende kommentarer fra samme bruger udløser det ikke igen.

### Kontekst agenten modtager

- Den nye kommentar.
- Valgfri tråd-/brugerhistorik-/sidekontekst som konfigureret.

Når brugerhistorik-konteksten er slået til, vil brugerens liste over nylige kommentarer naturligvis være tom (eller kun indeholde denne), men tillidsfaktoren og kontoens alder er udfyldt.

### Bemærk

- "Første kommentar på dette site" er scoped til **tenant**, ikke globalt på tværs af FastComments. En bruger med kommentarer på andre FastComments-sites udløser stadig denne trigger første gang, de poster på dit.
- Triggeren udløses kun for brugere med et userId. Anonyme uverificerede kommentarer uden et stabilt userId udløser den ikke.
- Triggeren udløses, når kommentaren er godkendt/synlig (ikke ved det oprindelige opslagstidspunkt). Redigeringer eller moderatorhandlinger på første kommentarer udløser den ikke igen.

### Almindelige anvendelser

- **Velkomsthilsen** - [Welcome Greeter-skabelonen](#template-welcome-greeter) er bygget omkring denne trigger.
- **Onboarding** - send en [DM-advarsel](#tool-warn-user) (bruges her som en venlig orientering snarere end en advarsel), der henviser brugeren til fællesskabets retningslinjer.
- **Underretning til anmelder** - hvis du vil have et menneske til at se på hver ny kommentators første opslag, kan [`mark_comment_reviewed`](#tools-overview) markere dem til gennemgang.

---