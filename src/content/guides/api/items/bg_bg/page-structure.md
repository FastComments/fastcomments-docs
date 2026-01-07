Обектът `Page` представлява страницата, към която могат да принадлежат много коментари. Тази връзка се определя от
`urlId`.

`Page` съхранява информация като заглавието на страницата, броя на коментарите и `urlId`.

Структурата на обекта Page е следната:

[inline-code-attrs-start title = 'Структура на Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** Setting this to null means all SSO users can see the page. An empty list means it is closed to all users. **/
    accessibleByGroupIds?: string[] | null
    /** Is this page closed for new comments? **/
    isClosed?: boolean
}
[inline-code-end]
