---
Объект `Page` представляет страницу, к которой могут относиться многие комментарии. Эта связь определяется
`urlId`.

Объект `Page` хранит информацию, такую как заголовок страницы, количество комментариев и `urlId`.

Структура объекта Page выглядит следующим образом:

[inline-code-attrs-start title = 'Структура страницы'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** Установка этого в null означает, что все пользователи SSO могут видеть страницу. Пустой список означает, что она закрыта для всех пользователей. **/
    accessibleByGroupIds?: string[] | null
    /** Закрыта ли эта страница для новых комментариев? **/
    isClosed?: boolean
}
[inline-code-end]

---