## Handlinger og Søgninger

Handlinger opretter data i FastComments; søgninger slår data op, så et senere trin kan bruge dem. Hver handling kalder FastComments REST‑API’en og bruger de samme API‑credits, som kaldet ville koste fra din egen kode: én credit pr. kald, medmindre andet er angivet.

## Opret Kommentar

Poster en kommentar på en side.

| Felt | Påkrævet | Bemærkninger |
|------|----------|--------------|
| Side‑URL‑ID | Ja | URL‑ID’en som kommentarfunktionen bruger på siden. Kommentarer grupperes efter den. |
| Side‑URL | Ja | Den fulde side‑URL, der bruges i notifikations‑e‑mails. |
| Kommentar | Ja | Kommentarens indhold i FastComments markdown. |
| Kommentators Navn | Ja | Navne er unikke pr. e‑mail, så genbrug af et navn med en anden e‑mail mislykkes. |
| Kommentators E‑mail | Nej | En bruger oprettes for e‑mailen, hvis den endnu ikke findes. |
| Bruger‑ID | Nej | Et eksisterende SSO‑bruger‑id. Har forrang over navn og e‑mail. |
| Forældrekommentar‑ID | Nej | Angiv for at oprette et svar. |
| Godkendt, Verificeret | Nej | Begge er som standard true. Ikke‑godkendte kommentarer forbliver skjulte indtil de modereres. |
| Udgivet På | Nej | Standard er nu. |
| Avatar‑URL, Side Titel, Locale | Nej | Locale er som standard `en_us`. |
| Vis Live i Widget | Nej | Skubber kommentaren til seere i realtid. Koster 2 credits i stedet for 1. |
| Kør Spam Tjek, Send E‑mails | Nej | Deaktiveret som standard. |

## Opret Side

Opretter en sidepost, før der findes nogen kommentarer på den, så den kan listes og begrænses. Tager URL‑ID, titel, URL og valgfrit de SSO‑gruppe‑ID’er, der må se den.

## Opret SSO‑bruger

Opretter en single sign‑on‑bruger. Tager dit eget bruger‑id, brugernavn og e‑mail samt valgfrit visningsnavn, visningsetiket, avatar, hjemmeside, gruppe‑ID’er og notifikations‑ og privatlivs‑flag. Administrative roller kan ikke tildeles fra Zapier.

## Opret Feed‑indlæg

Opretter et indlæg i et FastComments‑feed fra HTML‑indhold, med en valgfri titel, forfatter, tags og én link‑forhåndsvisning.

## Opret Hashtag

Opretter et hashtag, som kommentatorer kan bruge, med en valgfri URL, den linker til.

## Flag Kommentar

Flagger en kommentar til moderatorgennemgang. Angiv ID‑et på den bruger, der udfører flagningen, eller lad feltet stå tomt for at flagge som Zapier‑integration.

## Søgninger

| Søgning | Input | Returnerer |
|---------|-------|------------|
| Find Kommentar | Kommentar‑ID | Kommentaren, eller intet. |
| Find SSO‑bruger | E‑mail | SSO‑brugeren, eller intet. |
| Find Side | URL‑ID | Siden, eller intet. |

En søgning, der ikke finder noget, får ikke Zap’en til at fejle. Kombinér en søgning med en oprettelse i Zapier’s “find or create”-tilstand for at oprette siden eller brugeren, når den mangler.