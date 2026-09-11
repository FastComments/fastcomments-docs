## Acties en Zoekopdrachten

Acties maken gegevens aan in FastComments; zoekopdrachten zoeken gegevens op zodat een latere stap ze kan gebruiken. Elke actie roept de FastComments REST API aan en verbruikt dezelfde API‑credits die de oproep in je eigen code zou kosten: één credit per oproep, tenzij anders vermeld.

## Commentaar maken

Plaatst een commentaar op een pagina.

| Veld | Verplicht | Opmerkingen |
|------|-----------|-------------|
| Page URL ID | Ja | De URL‑ID die de commentaarwidget op de pagina gebruikt. Commentaren worden hierop gegroepeerd. |
| Page URL | Ja | De volledige pagin URL, gebruikt in notificatie‑e‑mails. |
| Comment | Ja | De commentaartekst in FastComments‑markdown. |
| Commenter Name | Ja | Namen zijn uniek per e‑mail, dus het hergebruiken van een naam met een ander e‑mailadres mislukt. |
| Commenter Email | Nee | Er wordt een gebruiker aangemaakt voor het e‑mailadres wanneer deze nog niet bestaat. |
| User ID | Nee | Een bestaand SSO‑gebruiker‑ID. Heeft voorrang boven de naam en het e‑mailadres. |
| Parent Comment ID | Nee | In te stellen om een antwoord te plaatsen. |
| Approved, Verified | Nee | Beide standaard op true. Niet‑goedgekeurde commentaren blijven verborgen tot ze gemodereerd worden. |
| Posted At | Nee | Standaard op nu. |
| Avatar URL, Page Title, Locale | Nee | Locale standaard op `en_us`. |
| Show Live In Widget | Nee | Stuurt het commentaar in realtime naar kijkers. Kost 2 credits in plaats van 1. |
| Run Spam Check, Send Emails | Nee | Standaard uitgeschakeld. |

## Pagina aanmaken

Maakt een paginarecord aan voordat er een commentaar op bestaat, zodat deze kan worden weergegeven en beperkt. Neemt de URL‑ID, titel, URL en optioneel de SSO‑groeps‑ID's die het mogen zien.

## SSO‑gebruiker aanmaken

Maakt een single sign‑on‑gebruiker aan. Neemt je eigen gebruikers‑ID, gebruikersnaam en e‑mail, plus optionele weergavenaam, weergavelabel, avatar, website, groeps‑ID's, en meldings‑ en privacy‑vlaggen. Administratieve rollen kunnen niet via Zapier worden toegekend.

## Feed‑bericht aanmaken

Maakt een bericht in een FastComments‑feed aan vanuit HTML‑inhoud. Het gebruikers‑ID van de auteur is vereist (een FastComments‑ of SSO‑gebruiker‑ID); titel, tags en één link‑preview zijn optioneel.

## Hashtag aanmaken

Maakt een hashtag aan die commentatoren kunnen gebruiken, met een optionele URL waarnaar deze linkt. Hashtags zijn uniek per account, dus een Zap die er elke keer een maakt, heeft iets unieks in de tag nodig.

## Commentaar markeren

Markeert een commentaar voor moderatie. Het ID van de gebruiker die markeert is vereist; het auteur‑ID dat door Commentaar maken wordt geretourneerd, werkt.

## Zoekopdrachten

| Zoekopdracht | Invoer | Resultaat |
|--------------|--------|-----------|
| Vind commentaar | Commentaar‑ID | Het commentaar, of niets. |
| Vind SSO‑gebruiker | E‑mail | De SSO‑gebruiker, of niets. |
| Vind pagina | URL‑ID | De pagina, of niets. |

Een zoekopdracht die niets vindt, laat de Zap niet falen. Combineer een zoekopdracht met een aanmaak in Zapier's "find or create"‑modus om de pagina of gebruiker aan te maken wanneer deze ontbreekt.