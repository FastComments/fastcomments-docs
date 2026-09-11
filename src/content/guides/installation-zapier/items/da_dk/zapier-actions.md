## Handlinger og Søgninger

Handlinger opretter data i FastComments; søgninger slår data op, så et senere trin kan bruge dem. Hver handling kalder FastComments REST API og bruger de samme API‑credits, som kaldet ville koste fra din egen kode: én credit pr. kald medmindre andet er angivet.

## Opret Kommentar

Sender en kommentar på en side.

| Felt | Påkrævet | Bemærkninger |
|------|----------|--------------|
| Side‑URL‑ID | Ja | URL‑ID'en som kommentarfunktionen bruger på siden. Kommentarer grupperes efter den. |
| Side‑URL | Ja | Den fulde side‑URL, der bruges i notifikations‑e‑mails. |
| Kommentar | Ja | Kommentarens indhold i FastComments markdown. |
| Kommentators Navn | Ja | Navne er unikke pr. e‑mail, så genbrug af et navn med en anden e‑mail mislykkes. |
| Kommentators E‑mail | Nej | En bruger oprettes for e‑mailen, hvis den endnu ikke findes. |
| Bruger‑ID | Nej | Et eksisterende SSO‑bruger‑ID. Har forrang over navn og e‑mail. |
| Forælder‑Kommentar‑ID | Nej | Angives for at poste et svar. |
| Godkendt, Verificeret | Nej | Begge er som standard true. Ikke‑godkendte kommentarer forbliver skjulte indtil de modereres. |
| Udgivet På | Nej | Standard er nu. |
| Avatar‑URL, Side‑Titel, Locale | Nej | Locale er som standard `en_us`. |
| Vis Live I Widget | Nej | Skubber kommentaren til seere i realtid. Koster 2 credits i stedet for 1. |
| Kør Spam‑Check, Send E‑mails | Nej | Deaktiveret som standard. |

## Opret Side

Opretter en sidepost, før der findes nogen kommentarer på den, så den kan listes og begrænses. Tager URL‑ID, titel, URL og valgfrit de SSO‑gruppe‑ID'er, der må se den.

## Opret SSO‑Bruger

Opretter en single sign‑on‑bruger. Tager dit eget bruger‑ID, brugernavn og e‑mail, plus valgfrit visningsnavn, visningsetiket, avatar, hjemmeside, gruppe‑ID'er samt notifikations‑ og privatlivs‑flag. Administrative roller kan ikke tildeles fra Zapier.

## Opret Feed‑Indlæg

Opretter et indlæg i et FastComments‑feed fra HTML‑indhold. Forfatterens bruger‑ID er påkrævet (et FastComments‑ eller SSO‑bruger‑ID); titel, tags og én link‑forhåndsvisning er valgfrie.

## Opret Hashtag

Opretter et hashtag, som kommentatorer kan bruge, med en valgfri URL, den linker til. Tags er unikke pr. konto, så en Zap, der opretter et på hver kørsel, har brug for noget unikt i tagget.

## Flag Kommentar

Flagger en kommentar til moderatorgennemgang. ID'et på den bruger, der udfører flaggingen, er påkrævet; forfatter‑ID'et returneret af Opret Kommentar fungerer.

## Søgninger

| Søgning | Input | Returnerer |
|---------|-------|------------|
| Find Kommentar | Kommentar‑ID | Kommentaren, eller intet. |
| Find SSO‑Bruger | E‑mail | SSO‑brugeren, eller intet. |
| Find Side | URL‑ID | Siden, eller intet. |

En søgning, der ikke finder noget, fejler ikke Zap’en. Kombinér en søgning med en oprettelse i Zapier’s “find or create”-tilstand for at oprette siden eller brugeren, når den mangler.