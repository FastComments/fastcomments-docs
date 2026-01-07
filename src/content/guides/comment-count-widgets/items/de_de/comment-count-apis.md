Es gibt mehrere Endpunkte, um die Zaehlungen zu erhalten, je nachdem, was Sie moechten und ob Sie sie von einem Browser, Server oder mit dem API SDK abrufen moechten.

## Oeffentliche Kommentarzaehlungen

Sie koennen die oeffentlichen Kommentarzaehlungen mit den oben genannten Widgets oder mit den APIs, die sie verwenden, abrufen. Diese APIs sind seit 2019 unveraendert geblieben und werden sich nie aendern.

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

Dies gibt eine Struktur wie folgt zurueck:

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

Die Eigenschaft `postfix` ist immer enthalten.

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

Dies gibt eine Struktur wie folgt zurueck:

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

Das `counts`-Objekt wird nur fuer Seiten mit Zaehlungen gefuellt. Die `translations`-Map ist immer vorhanden, da sie fuer das Widget verwendet wird.

### Verhalten oeffentlicher Endpunkte / Caching

Die oeffentlichen Endpunkte haben einen 60-Sekunden-Caching-Mechanismus, um Verkehrsspitzen zu bewaeltigen. Intern handelt es sich um einen Pro-Thread-LRU-Cache im Speicher des Servers, sodass Sie moeglicherweise sehen, dass sich die Zaehlungen leicht aendern (steigen und dann voruebergehend sinken), wenn viele Kommentare hinterlassen werden.

Die oeffentlichen Endpunkte geben immer die *gesamte* Kommentaranzahl zurueck, nicht die Wurzelkommentaranzahl.

### Server-seitige APIs / SDK

Der Weg, Kommentare von Ihrem Server zu erhalten, besteht darin, die [Pages API](/guide-api.html#page-structure) aufzurufen und das Seitenobjekt zu erhalten, das die Gesamtkommentaranzahl und die Wurzelkommentaranzahl enthaelt. Wir stellen SDKs bereit, mit denen Sie diese API aufrufen koennen, ohne die API-Anfrage manuell zu erstellen, und die typisierte Rueckgabewerte liefern.
