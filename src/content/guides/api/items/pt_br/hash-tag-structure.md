A `HashTag` object represents a tag that can be left by a user. HashTags can be used to link to an external piece of content or to
tie related comments together.

The structure for the `HashTag` object is as follows:

[inline-code-attrs-start title = 'Estrutura do HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Deve começar com "#" ou com o caractere desejado. **/
    tag: string
    /** Uma URL opcional para a qual a hashtag pode apontar. Em vez de filtrar comentários pela hashtag, a interface do usuário redirecionará para esta ao clicar. **/
    url?: string
    /** READONLY **/
    createdAt: string
}
[inline-code-end]

Notes:

- In some API endpoints you will see that the hashtag is used in the URL. Remember to URI-Encoded values. For example, `#` should instead be represented as `%23`.
- Some of these fields are marked `READONLY` - these are returned by the API but cannot be set.