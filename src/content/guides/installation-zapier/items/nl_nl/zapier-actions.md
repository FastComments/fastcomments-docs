## Acties en Zoekopdrachten

Acties maken data in FastComments; zoekopdrachten zoeken data op zodat een latere stap deze kan gebruiken. Elke actie roept de FastComments REST‑API aan en verbruikt dezelfde API‑credits als de oproep vanuit je eigen code: één credit per oproep tenzij anders vermeld.

## Reactie maken

Plaatst een reactie op een pagina.

| Field | Required | Notes |
|-------|----------|-------|
| Page URL ID | Ja | De URL‑ID die de reactiewidget op de pagina gebruikt. Reacties worden hierop gegroepeerd. |
| Page URL | Ja | De volledige paginanaam, gebruikt in notificatie‑e‑mails. |
| Comment | Ja | De reactietekst in FastComments‑markdown. |
| Commenter Name | Ja | Namen zijn uniek per e‑mail, dus het hergebruiken van een naam met een andere e‑mail mislukt. |
| Commenter Email | Nee | Er wordt een gebruiker aangemaakt voor de e‑mail wanneer deze nog niet bestaat. |
| User ID | Nee | Een bestaand SSO‑gebruiker‑id. Heeft voorrang boven de naam en e‑mail. |
| Parent Comment ID | Nee | In te stellen om een antwoord te plaatsen. |
| Approved, Verified | Nee | Beide standaard op true. Niet‑goedgekeurde reacties blijven verborgen tot moderatie. |
| Posted At | Nee | Standaard op nu. |
| Avatar URL, Page Title, Locale | Nee | Locale standaard `en_us`. |
| Show Live In Widget | Nee | Stuurt de reactie in realtime naar kijkers. Kost 2 credits in plaats van 1. |
| Run Spam Check, Send Emails | Nee | Standaard uitgeschakeld. |

## Pagina maken

Maakt een paginarecord aan voordat er een reactie op bestaat, zodat deze kan worden weergegeven en beperkt. Neemt de URL‑ID, titel, URL en eventueel de SSO‑groeps‑ids die het mogen zien.

## SSO‑gebruiker maken

Maakt een single sign‑on‑gebruiker aan. Neemt je eigen gebruikers‑id, gebruikersnaam en e‑mail, plus optioneel weergavenaam, label, avatar, website, groeps‑ids, en notificatie‑ en privacy‑vlaggen. Administratieve rollen kunnen niet via Zapier worden toegekend.

## Feedbericht maken

Maakt een bericht in een FastComments‑feed aan vanuit HTML‑inhoud, met een optionele titel, auteur, tags en één link‑preview.

## Hashtag maken

Maakt een hashtag aan die reageerders kunnen gebruiken, met een optionele URL waarnaar deze linkt.

## Reactie markeren

Markeert een reactie voor moderatie‑review. Geef de id van de gebruiker die de markering uitvoert, of laat leeg om te markeren als de Zapier‑integratie.

## Zoekopdrachten

| Search | Input | Returns |
|--------|-------|---------|
| Find Comment | Comment ID | De reactie, of niets. |
| Find SSO User | Email | De SSO‑gebruiker, of niets. |
| Find Page | URL ID | De pagina, of niets. |

Een zoekopdracht die niets vindt, laat de Zap niet falen. Combineer een zoekopdracht met een aanmaak in Zapier’s “find or create”‑modus om de pagina of gebruiker aan te maken wanneer deze ontbreekt.