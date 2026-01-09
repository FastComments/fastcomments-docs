---
A `Vote` 对象表示用户留下的投票。

评论与投票之间的关系通过 `commentId` 来定义。

Vote 对象的结构如下：

[inline-code-attrs-start title = 'Vote 对象结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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