Объект `Page` представляет страницу, которой может принадлежать множество комментариев. Эта связь определяется
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
    /** Установка этого в null означает, что страницу могут видеть все пользователи SSO. Пустой список означает, что доступ закрыт для всех пользователей. **/
    accessibleByGroupIds?: string[] | null
    /** Закрыта ли эта страница для новых комментариев? **/
    isClosed?: boolean
}
[inline-code-end]

---