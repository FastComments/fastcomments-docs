---
Um objeto `Vote` representa um voto deixado por um usuário.

A relação entre comentários e Vote é definida via `commentId`.

A estrutura para o objeto Vote é a seguinte:

[inline-code-attrs-start title = 'Estrutura do Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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