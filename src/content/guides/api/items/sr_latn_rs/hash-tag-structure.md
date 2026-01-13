Objekat `HashTag` predstavlja oznaku koju korisnik može ostaviti. HashTags se mogu koristiti za povezivanje sa spoljnim sadržajem ili za
povezivanje povezanih komentara.

Struktura `HashTag` objekta je sledeća:

[inline-code-attrs-start title = 'Struktura HashTag objekta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Treba da počinje sa "#" ili željenim karakterom. **/
    tag: string
    /** Opcioni URL na koji hashtag može da pokazuje. Umesto filtriranja komentara po hashtag-u, UI će preusmeriti na ovaj URL nakon klika. **/
    url?: string
    /** SAMO ZA ČITANJE **/
    createdAt: string
}
[inline-code-end]

Napomene:

- U nekim API endpoint-ima videćete da se hashtag koristi u URL-u. Zapamtite da treba URI-enkodirati vrednosti. Na primer, `#` bi trebalo da bude predstavljeno kao `%23`.
- Neki od ovih polja su označeni kao `READONLY` - ona se vraćaju iz API-ja, ali se ne mogu postaviti.