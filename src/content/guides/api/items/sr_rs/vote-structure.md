Објекат `Vote` представља глас који је оставио корисник.

Однос између коментара и гласа дефинисан је преко `commentId`.

Структура објекта Vote је следећа:

[inline-code-attrs-start title = 'Структура Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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