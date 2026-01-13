Обектът `Vote` представлява глас, оставен от потребител.

Връзката между коментари и глас се дефинира чрез `commentId`.

Структурата на обекта Vote е следната:

[inline-code-attrs-start title = 'Структура на Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Vote {
    id: string
    urlId: string
    commentId: string
    userId: string
    direction: 1 | -1
    createdAt: string
}
[inline-code-end]
