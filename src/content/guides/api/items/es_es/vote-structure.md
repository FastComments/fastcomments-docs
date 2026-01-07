Un objeto `Vote` representa un voto dejado por un usuario.

La relación entre comentarios y votos se define mediante `commentId`.

La estructura del objeto Vote es la siguiente:

[inline-code-attrs-start title = 'Estructura de Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
