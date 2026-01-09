---
Obiekt `Vote` reprezentuje głos oddany przez użytkownika.

Relacja między komentarzami a głosami jest określona za pomocą `commentId`.

Struktura obiektu `Vote` jest następująca:

[inline-code-attrs-start title = 'Struktura obiektu Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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