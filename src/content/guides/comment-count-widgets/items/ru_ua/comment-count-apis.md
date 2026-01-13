Существует несколько конечных точек для получения подсчетов, в зависимости от того, что вам нужно и хотите ли вы получить их из браузера, сервера или с помощью API SDK.

## Публичные подсчеты комментариев

Вы можете получить публичные подсчеты комментариев, используя виджеты выше или используя API, которые они используют. Эти API остаются неизменными с 2019 года и никогда не изменятся.

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

Это вернет структуру вида:

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

Свойство `postfix` всегда включено.

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

Это вернет структуру вида:

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

Объект `counts` заполняется только для страниц, которые имеют подсчеты. Карта `translations` всегда присутствует, так как используется для виджета.

### Поведение публичных конечных точек / Кэширование

Публичные конечные точки имеют 60-секундный механизм кэширования для обработки всплесков трафика. Внутренне это LRU-кэш на поток в памяти сервера, поэтому вы можете видеть, что подсчеты немного меняются (растут, а затем временно падают), когда люди оставляют много комментариев.

Публичные конечные точки всегда возвращают *общее* количество комментариев, а не количество корневых комментариев.

### Серверные API / SDK

Способ получить комментарии с вашего сервера - вызвать [Pages API](/guide-api.html#page-structure) и получить объект страницы, который содержит общее количество комментариев и количество корневых комментариев. Мы предоставляем SDK, которые позволяют вызывать этот API без ручного построения API-запроса и предоставляют типизированные возвращаемые значения.
