一個 `Page` 物件代表許多評論可能屬於的頁面。這種關係是由 `urlId` 定義的。

`Page` 儲存像是頁面標題、評論數量及 `urlId` 等資訊。

Page 物件的結構如下：

[inline-code-attrs-start title = '頁面結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** 將此設為 null 表示所有 SSO 使用者都可以看到該頁面。空清單表示該頁面對所有使用者關閉。 **/
    accessibleByGroupIds?: string[] | null
    /** 此頁面是否已關閉以接受新評論？ **/
    isClosed?: boolean
}
[inline-code-end]

---