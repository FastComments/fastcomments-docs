Der er nogle forskellige endpoints til at hente taelingerne, afhaengigt af hvad du oensker, og om du vil hente dem fra en browser, server eller ved hjaelp af API SDK'en.

## Offentlige kommentartaellinger

Du kan hente de offentlige kommentartaellinger ved hjaelp af widgets ovenfor eller ved hjaelp af de API'er, de bruger. Disse API'er er forblevet uaendrede siden 2019 og vil aldrig aendre sig.

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

Dette vil returnere en struktur som:

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

Egenskaben `postfix` er altid inkluderet.

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

Dette vil returnere en struktur som:

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

Objektet `counts` udfyldes kun for sider, der har taellinger. Kortet `translations` er altid til stede, da det bruges til widgetten.

### Offentlig endpoint-adfaerd / Caching

De offentlige endpoints har en 60-sekunders caching-mekanisme til at haandtere trafikspidsbelastninger. Internt er dette en per-traad LRU-cache i hukommelsen paa serveren, saa du kan se taellinger aendre sig lidt (gaa op og derefter midlertidigt ned), naar folk efterlader mange kommentarer.

De offentlige endpoints returnerer altid det *totale* antal kommentarer, ikke rodkommentarantallet.

### Server-side API'er / SDK

Maaden at hente kommentarer fra din server er at kalde [Pages API](/guide-api.html#page-structure) og hente sideobjektet, som indeholder det totale antal kommentarer og rodkommentarantallet. Vi leverer SDK'er, der giver dig mulighed for at kalde denne API uden at konstruere API-anmodningen manuelt og giver typede returvaerdier.
