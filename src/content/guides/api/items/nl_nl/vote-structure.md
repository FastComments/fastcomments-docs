Een `Vote`-object vertegenwoordigt een door een gebruiker achtergelaten stem.

De relatie tussen reacties en stemmen wordt gedefinieerd via `commentId`.

De structuur voor het `Vote`-object is als volgt:

[inline-code-attrs-start title = 'Structuur van Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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