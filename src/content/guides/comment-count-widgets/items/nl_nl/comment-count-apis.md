Er zijn een paar endpoints om de tellingen te krijgen, afhankelijk van wat u wilt en of u ze wilt ophalen vanuit een browser, server of met behulp van de API SDK.

## Openbare Commentaartellingen

U kunt de openbare commentaartellingen krijgen met behulp van de bovenstaande widgets of met behulp van de API's die ze gebruiken. Deze API's zijn onveranderd gebleven sinds 2019 en zullen nooit veranderen.

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

Dit zal een structuur retourneren zoals:

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

De `postfix` eigenschap is altijd opgenomen.

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

Dit zal een structuur retourneren zoals:

[inline-code-attrs-start title = 'Bulk Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "translations": {
        "t174": "0 Comments",
        "t175": "1 Comment",
        "t176": "[count] Comments"
    },
    "counts": {
        "x": 10
    }
}
[inline-code-end]

Het `counts` object wordt alleen gevuld voor pagina's die tellingen hebben. De `translations` map is altijd aanwezig omdat deze wordt gebruikt voor de widget.

### Gedrag van Openbare Endpoints / Caching

De openbare endpoints hebben een 60-seconden caching-mechanisme om pieken in verkeer aan te kunnen. Intern is dit een per-thread LRU-cache in het geheugen op de server, dus u kunt zien dat tellingen iets veranderen (omhoog gaan en dan tijdelijk weer omlaag) wanneer mensen veel reacties achterlaten.

De openbare endpoints retourneren altijd het *totale* aantal reacties, niet het aantal root-reacties.

### Server-Side API's / SDK

De manier om reacties van uw server te krijgen is door de [Pages API](/guide-api.html#page-structure) aan te roepen en het pagina-object te krijgen, dat het totale aantal reacties en het aantal root-reacties bevat. We bieden SDK's die u in staat stellen deze API aan te roepen zonder het API-verzoek handmatig te construeren en die getypeerde retourwaarden bieden.
