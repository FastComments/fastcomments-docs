Et `HashTag`-objekt repræsenterer et tag, der kan efterlades af en bruger. HashTags kan bruges til at linke til eksternt indhold eller til at
knytte relaterede kommentarer sammen.

Strukturen for `HashTag`-objektet er som følger:

[inline-code-attrs-start title = 'HashTag Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Should start with the "#" or desired character. **/
    tag: string
    /** An optional URL that the hashtag can point to. Instead of filtering comments by hashtag, the UI will redirect to this upon click. **/
    url?: string
    /** READONLY **/
    createdAt: string
}
[inline-code-end]

Bemærkninger:

- I nogle API-endpoints vil du se, at hashtagget bruges i URL'en. Husk at URI-kode værdier. For eksempel skal `#` i stedet repræsenteres som `%23`.
- Nogle af disse felter er markeret `READONLY` - disse returneres af API'et, men kan ikke sættes.
