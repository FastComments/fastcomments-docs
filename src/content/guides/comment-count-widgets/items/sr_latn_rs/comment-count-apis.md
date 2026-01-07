Postoji nekoliko krajnjih tacaka za dobijanje brojeva, u zavisnosti od toga sta zelite i da li zelite da ih dobijete iz pregledaca, servera ili koristeci API SDK.

## Javni brojevi komentara

Mozete dobiti javne brojeve komentara koristeci gornje vidzete ili koristeci API-je koje oni koriste. Ovi API-ji ostaju nepromenjeni od 2019. godine i nikada se nece promeniti.

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

Ovo ce vratiti strukturu kao:

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

Svojstvo `postfix` je uvek ukljuceno.

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

Ovo ce vratiti strukturu kao:

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

Objekat `counts` se popunjava samo za stranice koje imaju brojeve. Mapa `translations` je uvek prisutna jer se koristi za vidzet.

### Ponasanje javnih krajnjih tacaka / Kesiranje

Javne krajnje tacke imaju 60-sekundni mehanizam kesiranja za rukovanje naglim povecanjem saobracaja. Interno, ovo je LRU kes po niti u memoriji servera, tako da mozete videti da se brojevi malo menjaju (rastu pa privremeno padaju) kada ljudi ostavljaju mnogo komentara.

Javne krajnje tacke uvek vracaju *ukupan* broj komentara, a ne broj korenskih komentara.

### API-ji na strani servera / SDK

Nacin da dobijete komentare sa svog servera je da pozovete [Pages API](/guide-api.html#page-structure) i dobijete objekat stranice, koji sadrzi ukupan broj komentara i broj korenskih komentara. Pruzamo SDK-ove koji vam omogucavaju da pozovete ovaj API bez rucnog konstruisanja API zahteva i pruzaju tipizirane povratne vrednosti.
