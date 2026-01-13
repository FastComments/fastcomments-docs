A `Page` object represents the page that many comments may belong to. This relationship is defined by
`urlId`.

A `Page` stores information such as the page title, comment count, and `urlId`.

The structure for the Page object is as follows:

[inline-code-attrs-start title = 'ページ構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** これを null に設定すると、すべての SSO ユーザーがページを閲覧できます。空のリストは、すべてのユーザーに対して閉じられていることを意味します。 **/
    accessibleByGroupIds?: string[] | null
    /** このページは新しいコメントの受付を停止していますか？ **/
    isClosed?: boolean
}
[inline-code-end]

---