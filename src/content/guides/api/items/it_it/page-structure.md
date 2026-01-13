---
Un oggetto `Page` rappresenta la pagina a cui possono appartenere molti commenti. Questa relazione è definita da
`urlId`.

Un oggetto `Page` memorizza informazioni come il titolo della pagina, il numero di commenti e `urlId`.

La struttura per l'oggetto Page è la seguente:

[inline-code-attrs-start title = 'Struttura della Pagina'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** Impostare questo su null significa che tutti gli utenti SSO possono vedere la pagina. Una lista vuota significa che è chiusa a tutti gli utenti. **/
    accessibleByGroupIds?: string[] | null
    /** Questa pagina è chiusa ai nuovi commenti? **/
    isClosed?: boolean
}
[inline-code-end]

---