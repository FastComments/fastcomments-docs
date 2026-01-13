Obstaja nekaj koncnih tock za pridobivanje stevil, odvisno od tega, kaj zelite in ali jih zelite pridobiti iz brskalnika, streznika ali z uporabo API SDK.

## Javna stevila komentarjev

Javna stevila komentarjev lahko dobite z uporabo zgornjih pripomockov ali z uporabo API-jev, ki jih uporabljajo. Ti API-ji ostajajo nespremenjeni od leta 2019 in se nikoli ne bodo spremenili.

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

To bo vrnilo strukturo kot:

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

Lastnost `postfix` je vedno vkljucena.

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

To bo vrnilo strukturo kot:

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

Objekt `counts` je izpolnjen samo za strani, ki imajo stevila. Zemljevid `translations` je vedno prisoten, ker se uporablja za pripomoček.

### Obnasanje javnih koncnih tock / Predpomnjenje

Javne koncne tocke imajo 60-sekundni mehanizem predpomnjenja za obvladovanje konic v prometu. Interno je to LRU predpomnilnik na nit v pomnilniku streznika, tako da lahko vidite, da se stevila rahlo spreminjajo (narastejo in nato zacasno padejo), ko ljudje pustijo veliko komentarjev.

Javne koncne tocke vedno vrnejo *skupno* stevilo komentarjev, ne stevila korenskih komentarjev.

### API-ji na strani streznika / SDK

Nacin za pridobitev komentarjev s svojega streznika je klic [Pages API](/guide-api.html#page-structure) in pridobitev objekta strani, ki vsebuje skupno stevilo komentarjev in stevilo korenskih komentarjev. Zagotavljamo SDK-je, ki vam omogocajo klic tega API-ja brez rocne konstrukcije API zahteve in zagotavljajo tipizirane vrnjene vrednosti.
