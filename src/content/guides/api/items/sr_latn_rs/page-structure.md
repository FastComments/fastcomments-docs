Objekat `Page` predstavlja stranicu kojoj može pripadati mnogo komentara. Ovaj odnos je definisan preko `urlId`.

`Page` čuva informacije kao što su naslov stranice, broj komentara i `urlId`.

Struktura za Page objekat je sledeća:

[inline-code-attrs-start title = 'Struktura Page objekta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** Postavljanje na null znači da svi SSO korisnici mogu videti stranicu. Prazna lista znači da je zatvorena za sve korisnike. **/
    accessibleByGroupIds?: string[] | null
    /** Da li je ova stranica zatvorena za nove komentare? **/
    isClosed?: boolean
}
[inline-code-end]

---