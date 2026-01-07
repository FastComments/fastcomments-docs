Istnieje kilka punktow koncowych do uzyskania liczb, w zaleznosci od tego, czego potrzebujesz i czy chcesz je pobrac z przegladarki, serwera lub uzywajac SDK API.

## Publiczne liczby komentarzy

Mozesz uzyskac publiczne liczby komentarzy uzywajac powyzszych widgetow lub uzywajac API, ktore one wykorzystuja. Te API pozostaja niezmienione od 2019 roku i nigdy sie nie zmienia.

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

To zwroci strukture jak:

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

Wlasciwosc `postfix` jest zawsze dolaczona.

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

To zwroci strukture jak:

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

Obiekt `counts` jest wypelniany tylko dla stron, ktore maja liczby. Mapa `translations` jest zawsze obecna, poniewaz jest uzywana przez widget.

### Zachowanie publicznych punktow koncowych / Buforowanie

Publiczne punkty koncowe maja 60-sekundowy mechanizm buforowania do obslugi skokow ruchu. Wewnetrznie jest to pamiec podreczna LRU na watek w pamieci serwera, wiec mozesz zobaczyc, ze liczby nieznacznie sie zmieniaja (rosna, a potem tymczasowo spadaja), gdy ludzie zostawiaja wiele komentarzy.

Publiczne punkty koncowe zawsze zwracaja *calkowita* liczbe komentarzy, a nie liczbe komentarzy glownych.

### API po stronie serwera / SDK

Sposobem na uzyskanie komentarzy z serwera jest wywolanie [Pages API](/guide-api.html#page-structure) i pobranie obiektu strony, ktory zawiera calkowita liczbe komentarzy i liczbe komentarzy glownych. Dostarczamy SDK, ktore pozwalaja wywolac to API bez recznego konstruowania zadania API i zapewniaja typowane wartosci zwracane.
