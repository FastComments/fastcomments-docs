`Page` オブジェクトは、多くのコメントが属する可能性のあるページを表します。この関係は `urlId` によって定義されます。

`Page` は、ページタイトル、コメント数、`urlId` などの情報を保存します。

Page オブジェクトの構造は以下の通りです:

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
    /** Setting this to null means all SSO users can see the page. An empty list means it is closed to all users. **/
    accessibleByGroupIds?: string[] | null
    /** Is this page closed for new comments? **/
    isClosed?: boolean
}
[inline-code-end]