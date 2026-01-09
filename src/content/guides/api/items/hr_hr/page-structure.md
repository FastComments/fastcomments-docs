Objekt `Page` predstavlja stranicu kojoj može pripadati više komentara. Ovaj odnos definira se pomoću `urlId`.

`Page` sprema informacije kao što su naslov stranice, broj komentara i `urlId`.

Struktura za objekt `Page` je sljedeća:

[inline-code-attrs-start title = 'Struktura stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** Postavljanje ovog na null znači da svi SSO korisnici mogu vidjeti stranicu. Prazna lista znači da je zatvorena za sve korisnike. **/
    accessibleByGroupIds?: string[] | null
    /** Je li ova stranica zatvorena za nove komentare? **/
    isClosed?: boolean
}
[inline-code-end]

---