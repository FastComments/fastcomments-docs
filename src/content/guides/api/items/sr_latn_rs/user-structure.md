`User` je objekat koji predstavlja najčešći zajednički imenilac svih korisnika.

Imajte na umu da u FastComments imamo više različitih slučajeva upotrebe za korisnike:

- Secure SSO
- Simple SSO
- Tenant korisnici (na primer: Administratori)
- Komentatori

Ovaj API je za **Komentatore** i korisnike kreirane putem **Simple SSO**. U suštini, svaki korisnik kreiran preko vašeg sajta može se pristupiti putem ovog API-ja. Tenant korisnici takođe mogu biti dobijeni na ovaj način, ali ćete dobiti više informacija koristeći `/tenant-users/` API.

Za `Secure SSO` koristite `/sso-users/` API.

Ne možete izmeniti ove tipove korisnika. Oni su kreirali svoj nalog preko vašeg sajta, tako da pružamo osnovni pristup samo za čitanje, ali ne možete vršiti izmene. Ako želite imati ovaj tip toka - morate podesiti `Secure SSO`.

Struktura za the `User` objekat je sledeća:

[inline-code-attrs-start title = 'Struktura objekta User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** Ovo je takođe id koji se koristi kao userId na objektima komentara. **/
    id: string
    username: string
    /** Link ka blogu komentatora, na primer. **/
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