---
Objekt `Page` predstavlja stran, kateremu lahko pripada več komentarjev. Ta povezava je določena z
`urlId`.

Objekt `Page` hrani informacije, kot so naslov strani, število komentarjev in `urlId`.

Struktura objekta Page je naslednja:

[inline-code-attrs-start title = 'Struktura strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** Nastavitev na null pomeni, da lahko stran vidijo vsi SSO uporabniki. Prazen seznam pomeni, da je zaprt za vse uporabnike. **/
    accessibleByGroupIds?: string[] | null
    /** Je ta stran zaprta za nove komentarje? **/
    isClosed?: boolean
}
[inline-code-end]

---