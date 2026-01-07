Sayilari almak icin birkac endpoint vardir, ne istediginize ve bunlari bir tarayicidan, sunucudan veya API SDK kullanarak almak isteyip istemediginize bagli olarak.

## Genel Yorum Sayilari

Yukaridaki widget'lari kullanarak veya onlarin kullandigi API'leri kullanarak genel yorum sayilarini alabilirsiniz. Bu API'ler 2019'dan beri degismeden kalmistir ve asla degismeyecektir.

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

Bu, su sekilde bir yapi dondurecektir:

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

`postfix` ozelligi her zaman dahildir.

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

Bu, su sekilde bir yapi dondurecektir:

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

`counts` nesnesi yalnizca sayilari olan sayfalar icin doldurulur. `translations` haritasi widget icin kullanildigi icin her zaman mevcuttur.

### Genel Endpoint Davranisi / Onbellekleme

Genel endpoint'ler trafik artislarini yonetmek icin 60 saniyelik bir onbellekleme mekanizmasina sahiptir. Dahili olarak bu, sunucudaki bellekte is parcacigi basina bir LRU onbellegidir, bu nedenle insanlar cok sayida yorum biraktiginda sayilarin hafifce degistigini (yukselip sonra gecici olarak dustugunu) gorebilirsiniz.

Genel endpoint'ler her zaman kok yorum sayisini degil, *toplam* yorum sayisini dondurur.

### Sunucu Tarafi API'leri / SDK

Sunucunuzdan yorum almanin yolu, [Pages API](/guide-api.html#page-structure)'yi cagirmak ve toplam yorum sayisini ve kok yorum sayisini iceren sayfa nesnesini almaktir. API istegini manuel olarak olusturmadan bu API'yi cagirmaniza olanak taniyan ve tipli donus degerleri saglayan SDK'lar sunuyoruz.
