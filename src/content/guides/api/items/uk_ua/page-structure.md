---
Об'єкт `Page` представляє сторінку, якій можуть належати багато коментарів. Це відношення визначається `urlId`.

Об'єкт `Page` зберігає інформацію, таку як заголовок сторінки, кількість коментарів та `urlId`.

Структура об'єкта Page виглядає так:

[inline-code-attrs-start title = 'Структура сторінки'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** Якщо встановити це в null, усі користувачі SSO зможуть бачити сторінку. Порожній список означає, що вона закрита для всіх користувачів. **/
    accessibleByGroupIds?: string[] | null
    /** Чи закрита ця сторінка для нових коментарів? **/
    isClosed?: boolean
}
[inline-code-end]

---