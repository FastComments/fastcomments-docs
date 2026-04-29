Udløses når netto-stemmeantallet for en kommentar når den konfigurerede tærskel. Netto-stemmer er `votesUp - votesDown`.

### Påkrævet konfiguration

- **Stemmetærskel** - heltal >= 1. Udløseren affyres på den stemme, der bringer netto-stemmeantallet til præcis dette tal.

Hvis tærsklen er 10 og en kommentar går fra 9 til 10 i netto-stemmer, udløses triggeren én gang. Hvis en stemme derefter tager den fra 10 til 11, udløses triggeren **ikke** igen - den udløses ikke gentagne gange for hver ekstra stemme over tærsklen.

### Kontekst agenten modtager

- Kommentaren med aktuelle stemmetal.
- Den **stemmeretning** (`up` eller `down`) for den stemme, der udløste tærskelkrydset.
- Valgfri tråd-/brugerhistorik-/sidekontekst som konfigureret.

### Bemærk

- En kommentar, der stiger til 10, falder tilbage til 9 og stiger til 10 igen, vil udløse triggeren to gange. Der er ingen per-kommentar "fired once"-tilstand - hvis du har brug for den semantik, lad agenten oprette en [memory note](#tools-overview) ved første kørsel og tjekke for den ved efterfølgende kørsler.
- Tærsklen er altid et **netto** stemmetal, ikke rå opstemmer. En kommentar med 12 up og 2 down har netto 10 og udløser triggeren; en med 10 up og 0 down udløser også.
- Krydsninger forårsaget kun af nedstemmer er mulige - en kommentar, der går fra 11 til 10 på grund af en nedstemning, udløser også. `voteDirection`-parameteren i konteksten fortæller agenten, hvilken retning tærskelkrydset kom fra.

### Almindelige anvendelser

- **Fastgørelse** - [Top Comment Pinner template](#template-top-comment-pinner) er bygget omkring denne trigger.
- **Promovering / arbejdsgange for fremhævede kommentarer** - udsend en begivenhed via [Webhooks](#webhooks-overview), så et eksternt system kan promovere kommentaren andre steder på dit site.
- **Engagement-tracking** - registrer en memory note om brugeren, der skrev kommentaren, så andre agenter ved, at de har produceret populært indhold.

### Finjustering

Den rette tærskel er specifik for dit community. Overvåg [Run History](#run-history) i et par dage med en lav tærskel (5) for at se, hvor ofte den udløses. Hæv tærsklen, indtil udløsningsfrekvensen matcher den frekvens, du faktisk ønsker.