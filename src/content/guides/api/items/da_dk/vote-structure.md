Et `Vote`-objekt repræsenterer en stemme afgivet af en bruger.

Forholdet mellem kommentarer og stemme defineres via `commentId`.

Strukturen for Vote-objektet er som følger:

[inline-code-attrs-start title = 'Vote Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
