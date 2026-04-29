**Sjabloon-ID:** `welcome_greeter`

De Welcome Greeter reageert hartelijk op personen die voor het eerst reageren. Het is het minst risicovolle sjabloon (geen destructieve tools) en een goede eerste agent om live te zetten.

### Ingebouwde initiële prompt

[inline-code-attrs-start title = 'Initiële prompt voor Welcome Greeter-sjabloon'; type='text' inline-code-attrs-end]
[inline-code-start]
Je bent een warme community-gastheer. Reageer op mensen die voor het eerst reageren met een korte, persoonlijke welkomsgroet. Noem één specifiek ding uit hun reactie zodat het niet als een sjabloon overkomt. Beperk reacties tot 1–2 zinnen. Reageer nooit op accounts die ouder zijn dan 24 uur.
[inline-code-end]

### Triggers

- **Nieuwe gebruiker plaatst hun eerste reactie op deze site** (`NEW_USER_FIRST_COMMENT`).

Deze gebeurtenis wordt precies één keer per gebruiker geactiveerd, dus de agent kan niet blijven herhalen. Zie [Trigger: New User First Comment](#trigger-new-user-first-comment).

### Toegestane tools

- [`write_comment`](#tools-overview)

Dat is het enige hulpmiddel — de agent kan letterlijk niet modereren, stemmen, verbannen of DM'en.

### Aanbevolen toevoegingen voordat je live gaat

- **Stel de Display name in** op iets uitnodigends — "Community Bot", je site-mascotte of je merknaam. De weergavenaam is wat lezers zien bij de welkomstreactie.
- **Vink "Pagina-titel, subtitel, beschrijving en meta-tags opnemen" aan** in [Context Options](#context-options). De reacties van de greeter worden merkbaar beter als hij kan verwijzen naar waar de pagina over gaat.
- **Overweeg locatierestricties** als je in meerdere talen opereert. Een welkomstantwoord in de verkeerde taal is storender dan een gemist antwoord. Zie [Scope: URL and Locale Filters](#scope-url-locale).

### Waarom geen goedkeuringen nodig zijn

De agent schrijft alleen nieuwe reacties en alleen als reactie op een eenmalige trigger. In het ergste geval: een ongemakkelijke begroeting. Er is geen destructieve handeling die beveiligd moet worden. De meeste beheerders laten deze zonder enige goedkeuring draaien zodra een test-run er schoon uitziet.

---