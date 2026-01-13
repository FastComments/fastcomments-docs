Існує кілька кінцевих точок для отримання підрахунків, залежно від того, що вам потрібно і чи хочете ви отримати їх з браузера, сервера або за допомогою API SDK.

## Публічні підрахунки коментарів

Ви можете отримати публічні підрахунки коментарів, використовуючи віджети вище або використовуючи API, які вони використовують. Ці API залишаються незмінними з 2019 року і ніколи не зміняться.

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

Це поверне структуру виду:

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

Властивість `postfix` завжди включена.

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

Це поверне структуру виду:

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

Об'єкт `counts` заповнюється лише для сторінок, які мають підрахунки. Карта `translations` завжди присутня, оскільки використовується для віджета.

### Поведінка публічних кінцевих точок / Кешування

Публічні кінцеві точки мають 60-секундний механізм кешування для обробки сплесків трафіку. Внутрішньо це LRU-кеш на потік у пам'яті сервера, тому ви можете бачити, що підрахунки трохи змінюються (зростають, а потім тимчасово падають), коли люди залишають багато коментарів.

Публічні кінцеві точки завжди повертають *загальну* кількість коментарів, а не кількість кореневих коментарів.

### Серверні API / SDK

Спосіб отримати коментарі з вашого сервера - викликати [Pages API](/guide-api.html#page-structure) та отримати об'єкт сторінки, який містить загальну кількість коментарів та кількість кореневих коментарів. Ми надаємо SDK, які дозволяють викликати цей API без ручного побудови API-запиту та надають типізовані повернені значення.
