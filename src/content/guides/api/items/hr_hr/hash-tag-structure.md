Objekt `HashTag` predstavlja oznaku (tag) koju korisnik može ostaviti. HashTagovi se mogu koristiti za povezivanje s vanjskim sadržajem ili za
povezivanje srodnih komentara.

Struktura objekta `HashTag` je sljedeća:

[inline-code-attrs-start title = 'Struktura HashTaga'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Treba početi s znakom "#" ili željenim znakom. **/
    tag: string
    /** Opcionalni URL na koji hashtag može upućivati. Umjesto filtriranja komentara po hashtagu, UI će pri kliku preusmjeriti na ovaj URL. **/
    url?: string
    /** READONLY **/
    createdAt: string
}
[inline-code-end]

Napomene:

- U nekim API endpointima vidjet ćete da se hashtag koristi u URL-u. Ne zaboravite URI-kodirane vrijednosti. Na primjer, `#` treba umjesto toga predstaviti kao `%23`.
- Neka od ovih polja označena su kao `READONLY` - API ih vraća, ali se ne mogu postaviti.