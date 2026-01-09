`User` je objekt koji predstavlja najčešći zajednički nazivnik svih korisnika.

Imajte na umu da u FastComments imamo nekoliko različitih slučajeva upotrebe za korisnike:

- Secure SSO
- Simple SSO
- Tenant Users (Na primjer: Administratori)
- Commenters

Ovaj API je za **Komentatore** i korisnike stvorene putem **Simple SSO**. U osnovi, svaki korisnik stvoren putem vaše stranice može se dohvatiti putem ovog API-ja. Tenant Users se također mogu dohvatiti na ovaj način, ali ćete dobiti više informacija interakcijom s `/tenant-users/` API-jem.

Za `Secure SSO` molimo koristite `/sso-users/` API.

Ne možete ažurirati ove vrste korisnika. Oni su kreirali svoj račun putem vaše stranice, stoga pružamo osnovni pristup samo za čitanje, ali ne možete napraviti promjene. Ako želite imati takav tijek - morate postaviti `Secure SSO`.

Struktura za objekt `User` je sljedeća:

[inline-code-attrs-start title = 'Struktura objekta User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** Ovo je također id koji se koristi kao userId na objektima komentara. **/
    id: string
    username: string
    /** Na primjer, poveznica na blog komentatora. **/
    websiteUrl?: string
    email: string
    signUpDate: number
    createdFromUrlId: string
    createdFromTenantId: string
    avatarSrc?: string
    locale: FastCommentsLocale
    displayLabel?: string
    karma?: number
}
[inline-code-end]