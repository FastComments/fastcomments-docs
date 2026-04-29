**Sjabloon-ID:** `welcome_greeter`

De Welcome Greeter reageert hartelijk op mensen die voor het eerst reageren. Het is het minst risicovolle sjabloon (geen destructieve tools) en een goede eerste agent om live te zetten.

### Triggers

- **Nieuwe gebruiker plaatst voor het eerst een reactie op deze site** (`NEW_USER_FIRST_COMMENT`).

Deze gebeurtenis wordt precies één keer per gebruiker geactiveerd, dus de agent kan niet in een lus terechtkomen. Zie [Trigger: Nieuwe gebruiker - eerste reactie](#trigger-new-user-first-comment).

### Toegestane tools

- [`write_comment`](#tools-overview)

Dat is het enige hulpmiddel - de agent kan letterlijk niet modereren, stemmen, verbannen of privéberichten sturen.

### Aanbevolen toevoegingen voordat u live gaat

- **Stel de weergavenaam in** op iets uitnodigends - "Community Bot", uw site-mascotte of uw merknaam. De weergavenaam is wat lezers zien gekoppeld aan het welkomstantwoord.
- **Vink "Inclusief paginatitel, subtitel, beschrijving en meta-tags" aan** in [Context Options](#context-options). De antwoorden van de greeter worden merkbaar beter wanneer hij kan verwijzen naar waar de pagina daadwerkelijk over gaat.
- **Overweeg locale-beperkingen** als u in meerdere talen opereert. Een welkomstreactie in de verkeerde taal is meer storend dan een gemiste reactie. Zie [Scope: URL- en locale-filters](#scope-url-locale).

### Waarom geen goedkeuringen nodig zijn

De agent schrijft alleen nieuwe reacties en alleen bij een eenmalige trigger. In het slechtste geval: een onhandige begroeting. Er is geen destructieve actie die afgebakend moet worden. De meeste beheerders draaien deze zonder enige goedkeuring zodra een proefrun er schoon uitziet.