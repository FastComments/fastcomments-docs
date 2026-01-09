Об'єкт `Vote` представляє голос, залишений користувачем.

Взаємозв'язок між коментарями та голосом визначається через `commentId`.

Структура об'єкта Vote виглядає так:

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