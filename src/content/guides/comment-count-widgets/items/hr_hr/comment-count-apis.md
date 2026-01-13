Postoji nekoliko krajnjih tocaka za dobivanje brojeva, ovisno o tome sto zelite i zelite li ih dobiti iz preglednika, servera ili koristeci API SDK.

## Javni brojevi komentara

Mozete dobiti javne brojeve komentara koristeci widgete iznad ili koristeci API-je koje oni koriste. Ovi API-ji ostaju nepromijenjeni od 2019. godine i nikada se nece promijeniti.

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

Ovo ce vratiti strukturu poput:

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

Svojstvo `postfix` je uvijek ukljuceno.

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

Ovo ce vratiti strukturu poput:

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

Objekt `counts` popunjava se samo za stranice koje imaju brojeve. Mapa `translations` je uvijek prisutna jer se koristi za widget.

### Ponasanje javnih krajnjih tocaka / Kesiranje

Javne krajnje tocke imaju 60-sekundni mehanizam kesiranja za rukovanje naglim povecanjem prometa. Interno, ovo je LRU kes po niti u memoriji servera, pa mozete vidjeti da se brojevi malo mijenjaju (rastu pa privremeno padaju) kada ljudi ostavljaju puno komentara.

Javne krajnje tocke uvijek vracaju *ukupan* broj komentara, a ne broj korijenskih komentara.

### Server-side API-ji / SDK

Nacin da dobijete komentare s vaseg servera je da pozovete [Pages API](/guide-api.html#page-structure) i dobijete objekt stranice, koji sadrzi ukupan broj komentara i broj korijenskih komentara. Pruzamo SDK-ove koji vam omogucuju pozivanje ovog API-ja bez rucnog konstruiranja API zahtjeva i pruzaju tipizirane povratne vrijednosti.
