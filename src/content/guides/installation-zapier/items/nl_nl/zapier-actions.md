## Acties en Zoekopdrachten

Acties maken data aan in FastComments; zoekopdrachten zoeken data op zodat een latere stap deze kan gebruiken. Elke actie roept de FastComments REST API aan en verbruikt dezelfde API‑credits die de oproep vanuit je eigen code zou kosten: één credit per oproep tenzij anders vermeld.

## Commentaar Aanmaken

Plaatst een commentaar op een pagina.

| Veld | Verplicht | Opmerkingen |
|------|-----------|-------------|
| Pagina‑URL‑ID | Ja | De URL‑ID die de commentaarwidget op de pagina gebruikt. Commentaren worden hierop gegroepeerd. |
| Pagina‑URL | Ja | De volledige pagina‑URL, gebruikt in notificatie‑e‑mails. |
| Commentaar | Ja | De commentaartekst in FastComments markdown. |
| Naam van de commentator | Ja | Namen zijn uniek per e‑mail, dus het hergebruiken van een naam met een andere e‑mail mislukt. |
| E‑mail van de commentator | Nee | Er wordt een gebruiker aangemaakt voor het e‑mailadres wanneer deze nog niet bestaat. |
| Gebruikers‑ID | Nee | Een bestaand SSO‑gebruikers‑ID. Heeft voorrang boven de naam en e‑mail. |
| Bovenliggend commentaar‑ID | Nee | Instellen om een antwoord te plaatsen. |
| Goedgekeurd, Geverifieerd | Nee | Beide standaard op true. Niet‑goedgekeurde commentaren blijven verborgen tot moderatie. |
| Geplaatst op | Nee | Standaard op nu. |
| Avatar‑URL, Paginatitel, Taalinstelling | Nee | Taalinstelling standaard `en_us`. |
| Live weergeven in widget | Nee | Stuurt het commentaar in realtime naar kijkers. Kost 2 credits in plaats van 1. |
| Spamcontrole uitvoeren, E‑mails verzenden | Nee | Standaard uitgeschakeld. |

## Pagina Aanmaken of Bijwerken

Maakt een paginarecord aan voordat er commentaren op bestaan, zodat deze kan worden weergegeven en beperkt. Neemt de URL‑ID, titel, URL en optioneel de SSO‑groeps‑IDs die het mogen zien. Als een pagina met die URL‑ID al bestaat, wordt deze bijgewerkt met de opgegeven velden, zodat een Zap herhaaldelijk voor dezelfde pagina kan draaien.

## SSO‑gebruiker Aanmaken of Bijwerken

Maakt een single sign‑on‑gebruiker aan. Neemt je eigen gebruikers‑ID, gebruikersnaam en e‑mail, plus optioneel weergavenaam, weergavelabel, avatar, website, groeps‑IDs, en notificatie‑ en privacy‑vlaggen. Als een gebruiker met dat ID al bestaat, wordt deze in plaats daarvan bijgewerkt. Administratieve rollen kunnen niet via Zapier worden toegekend.

## Feed‑bericht Aanmaken

Maakt een bericht in een FastComments‑feed aan vanuit HTML‑inhoud. Het auteur‑gebruikers‑ID is vereist (een FastComments‑ of SSO‑gebruikers‑ID); titel, tags en één link‑preview zijn optioneel.

## Hashtag Aanmaken of Bijwerken

Maakt een hashtag aan die commentatoren kunnen gebruiken, met een optionele URL waarnaar deze linkt. Als de tag al bestaat, wordt deze bijgewerkt.

## Commentaar Markeren

Markeert een commentaar voor moderatie‑review. Het ID van de gebruiker die de markering uitvoert is vereist; het auteur‑ID dat wordt geretourneerd door **Create Comment** werkt.

## Zoekopdrachten

| Zoekopdracht | Invoer | Retourneert |
|--------------|--------|-------------|
| Vind Commentaar | Commentaar‑ID | Het commentaar, of niets. |
| Vind SSO‑gebruiker | E‑mail | De SSO‑gebruiker, of niets. |
| Vind Pagina | URL‑ID | De pagina, of niets. |

Een zoekopdracht die niets vindt, veroorzaakt geen fout in de Zap. **Find SSO User** en **Find Page** bieden Zapier’s “create if it doesn’t exist”‑optie, die de bijbehorende aanmaak uitvoert wanneer er niets wordt gevonden.