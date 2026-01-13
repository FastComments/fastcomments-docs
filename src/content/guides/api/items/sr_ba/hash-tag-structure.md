---
Objekat `HashTag` predstavlja oznaku koju korisnik može ostaviti. HashTags se mogu koristiti za povezivanje sa vanjskim sadržajem ili za povezivanje povezanih komentara.

The structure for the `HashTag` object is as follows:

[inline-code-attrs-start title = 'Struktura HashTag-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Treba da počinje sa "#" ili željenim znakom. **/
    tag: string
    /** Opcionalni URL na koji hashtag može upućivati. Umjesto filtriranja komentara po hashtagu, korisnički interfejs će pri kliku preusmjeriti na ovaj URL. **/
    url?: string
    /** SAMO ZA ČITANJE **/
    createdAt: string
}
[inline-code-end]

Notes:

- In some API endpoints you will see that the hashtag is used in the URL. Remember to URI-Encoded values. For example, `#` should instead be represented as `%23`.
- Some of these fields are marked `READONLY` - these are returned by the API but cannot be set.