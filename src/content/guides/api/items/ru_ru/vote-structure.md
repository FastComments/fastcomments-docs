---
Объект `Vote` представляет собой голос, оставленный пользователем.

Связь между комментариями и vote определяется через `commentId`.

Структура объекта `Vote` следующая:

[inline-code-attrs-start title = 'Структура Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---