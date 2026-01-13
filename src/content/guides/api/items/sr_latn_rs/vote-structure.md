Objekat `Vote` predstavlja glas koji je ostavio korisnik.

Veza između komentara i glasa definiše se preko `commentId`.

Struktura objekta Vote je sledeća:

[inline-code-attrs-start title = 'Struktura objekta Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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