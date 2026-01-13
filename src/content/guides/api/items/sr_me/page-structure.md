Објекат `Page` представља страницу којој може припадати много коментара. Овај однос је дефинисан помоћу
`urlId`.

`Page` чува информације као што су наслов странице, број коментара и `urlId`.

Структура објекта Page је следећа:

[inline-code-attrs-start title = 'Структура странице'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** Постављање ове вредности на null значи да сви SSO корисници могу видети страницу. Празна листа значи да је затворена за све кориснике. **/
    accessibleByGroupIds?: string[] | null
    /** Да ли је ова страница затворена за нове коментаре? **/
    isClosed?: boolean
}
[inline-code-end]