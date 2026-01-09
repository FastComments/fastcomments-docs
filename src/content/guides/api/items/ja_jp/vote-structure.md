---
A `Vote` object represents a vote left by a user.

コメントと投票の関係は `commentId` によって定義されます。

`Vote` オブジェクトの構造は次のとおりです:

[inline-code-attrs-start title = 'Vote オブジェクトの構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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