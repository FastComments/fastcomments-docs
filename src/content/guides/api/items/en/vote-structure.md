A `Vote` object represents a vote left by a user.

The relationship between comments and vote is defined via `commentId`.

The structure for the Vote object is as follows:

[inline-code-attrs-start title = 'Vote Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
