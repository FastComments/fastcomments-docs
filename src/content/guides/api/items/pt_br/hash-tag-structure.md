Um objeto `HashTag` representa uma tag que pode ser deixada por um usuário. HashTags podem ser usadas para vincular a um conteúdo externo ou para
conectar comentários relacionados.

A estrutura do objeto `HashTag` é a seguinte:

[inline-code-attrs-start title = 'Estrutura de HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Deve começar com o caractere "#" ou o desejado. **/
    tag: string
    /** Uma URL opcional para a qual a hashtag pode apontar. Em vez de filtrar comentários por hashtag, a UI redirecionará para esta URL ao clicar. **/
    url?: string
    /** READONLY **/
    createdAt: string
}
[inline-code-end]

Notas:

- Em alguns endpoints da API você verá que a hashtag é usada na URL. Lembre-se de usar valores com URI-Encoded. Por exemplo, `#` deve ser representado como `%23`.
- Alguns desses campos estão marcados como `READONLY` - eles são retornados pela API, mas não podem ser definidos.