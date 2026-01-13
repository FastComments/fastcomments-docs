---
Bir `Vote` nesnesi, bir kullanıcı tarafından bırakılan bir oyu temsil eder.

Yorumlar ile oy arasındaki ilişki `commentId` aracılığıyla tanımlanır.

`Vote` nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'Vote Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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