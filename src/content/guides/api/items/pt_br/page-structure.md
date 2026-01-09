Um `Page` object representa a página à qual muitos comentários podem pertencer. Esse relacionamento é definido por
`urlId`.

Um `Page` armazena informações como o título da página, a contagem de comentários e `urlId`.

A estrutura do objeto Page é a seguinte:

[inline-code-attrs-start title = 'Estrutura da Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** Definir isto como null significa que todos os usuários SSO podem ver a página. Uma lista vazia significa que está fechada para todos os usuários. **/
    accessibleByGroupIds?: string[] | null
    /** Esta página está fechada para novos comentários? **/
    isClosed?: boolean
}
[inline-code-end]

---