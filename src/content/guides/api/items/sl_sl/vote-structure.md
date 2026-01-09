Objekt `Vote` predstavlja glas, ki ga je oddal uporabnik.

Razmerje med komentarji in glasovi je določeno preko `commentId`.

Struktura za objekt `Vote` je naslednja:

[inline-code-attrs-start title = 'Struktura glasovanja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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