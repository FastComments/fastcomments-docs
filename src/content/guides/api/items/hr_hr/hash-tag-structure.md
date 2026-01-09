Objekt `HashTag` predstavlja oznaku koju može ostaviti korisnik. HashTagovi se mogu koristiti za povezivanje na vanjski sadržaj ili za povezivanje srodnih komentara.

Struktura za objekt `HashTag` je sljedeća:

[inline-code-attrs-start title = 'Struktura HashTag-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Treba počinjati znakom "#" ili željenim znakom. **/
    tag: string
    /** Opcionalni URL na koji hashtag može pokazivati. Umjesto filtriranja komentara po hashtag-u, korisničko sučelje će pri kliku preusmjeriti na ovu adresu. **/
    url?: string
    /** SAMO ZA ČITANJE **/
    createdAt: string
}
[inline-code-end]

Notes:

- In some API endpoints you will see that the hashtag is used in the URL. Remember to URI-Encoded values. For example, `#` should instead be represented as `%23`.
- Some of these fields are marked `READONLY` - these are returned by the API but cannot be set.