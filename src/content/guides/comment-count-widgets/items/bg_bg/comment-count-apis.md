Има няколко крайни точки за получаване на бройките, в зависимост от това какво искате и дали искате да ги получите от браузър, сървър или използвайки API SDK.

## Публични бройки коментари

Можете да получите публичните бройки коментари, използвайки уиджетите по-горе или използвайки API-тата, които те използват. Тези API-та остават непроменени от 2019 г. и никога няма да се променят.

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

Това ще върне структура като:

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

Свойството `postfix` винаги е включено.

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

Това ще върне структура като:

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

Обектът `counts` се попълва само за страници, които имат бройки. Картата `translations` винаги присъства, тъй като се използва за уиджета.

### Поведение на публичните крайни точки / Кеширане

Публичните крайни точки имат 60-секунден механизъм за кеширане за справяне с пикове в трафика. Вътрешно това е LRU кеш на нишка в паметта на сървъра, така че може да видите бройките да се променят леко (да се покачват и след това временно да падат), когато хората оставят много коментари.

Публичните крайни точки винаги връщат *общия* брой коментари, а не броя на основните коментари.

### API-та от страна на сървъра / SDK

Начинът да получите коментари от вашия сървър е да извикате [Pages API](/guide-api.html#page-structure) и да получите обекта на страницата, който съдържа общия брой коментари и броя на основните коментари. Ние предоставяме SDK-та, които ви позволяват да извикате този API без да конструирате API заявката ръчно и предоставят типизирани върнати стойности.
