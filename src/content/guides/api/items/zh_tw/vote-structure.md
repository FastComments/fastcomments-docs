一個 `Vote` 物件代表使用者所留下的投票。

留言與投票之間的關係由 `commentId` 定義。

Vote 物件的結構如下：

[inline-code-attrs-start title = 'Vote 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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