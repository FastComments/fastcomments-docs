A `Page` 对象表示许多评论可能属于的页面。此关系由
`urlId` 定义。

A `Page` 存储信息，例如页面标题、评论计数和 `urlId`。

Page 对象的结构如下：

[inline-code-attrs-start title = '页面结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** 将此设置为 null 意味着所有 SSO 用户都可以看到该页面。空列表表示对所有用户关闭。 **/
    accessibleByGroupIds?: string[] | null
    /** 此页面是否关闭以添加新评论？ **/
    isClosed?: boolean
}
[inline-code-end]

---