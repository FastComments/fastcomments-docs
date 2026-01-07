Постоји неколико крајњих тачака за добијање бројева, у зависности од тога шта желите и да ли желите да их добијете из прегледача, сервера или користећи API SDK.

## Јавни бројеви коментара

Можете добити јавне бројеве коментара користећи горње виџете или користећи API-је које они користе. Ови API-ји остају непромењени од 2019. године и никада се неће променити.

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

Ово ће вратити структуру као:

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

Својство `postfix` је увек укључено.

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

Ово ће вратити структуру као:

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

Објекат `counts` се попуњава само за странице које имају бројеве. Мапа `translations` је увек присутна јер се користи за виџет.

### Понашање јавних крајњих тачака / Кеширање

Јавне крајње тачке имају 60-секундни механизам кеширања за руковање наглим повећањем саобраћаја. Интерно, ово је LRU кеш по нити у меморији сервера, тако да можете видети да се бројеви мало мењају (расту па привремено падају) када људи остављају много коментара.

Јавне крајње тачке увек враћају *укупан* број коментара, а не број коренских коментара.

### API-ји на страни сервера / SDK

Начин да добијете коментаре са свог сервера је да позовете [Pages API](/guide-api.html#page-structure) и добијете објекат странице, који садржи укупан број коментара и број коренских коментара. Пружамо SDK-ове који вам омогућавају да позовете овај API без ручног конструисања API захтева и пружају типизиране повратне вредности.
