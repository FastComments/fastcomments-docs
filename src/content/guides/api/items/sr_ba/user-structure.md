`User` je objekat koji predstavlja najčešći zajednički imenitelj svih korisnika.

Imajte na umu da u FastComments imamo više različitih slučajeva upotrebe za korisnike:

- Secure SSO
- Simple SSO
- Tenant Users (For example: Administrators)
- Commenters

Ovaj API je za **Commenters** i korisnike kreirane putem **Simple SSO**. U suštini, bilo koji korisnik kreiran kroz vašu stranicu može se pristupiti putem ovog API-ja. Tenant Users se također mogu dohvatiti na ovaj način, ali ćete dobiti više informacija koristeći `/tenant-users/` API.

Za `Secure SSO` koristite `/sso-users/` API.

Ne možete ažurirati ove tipove korisnika. Oni su kreirali svoj nalog kroz vašu stranicu, pa pružamo osnovni pristup samo za čitanje, ali ne možete praviti izmjene. Ako želite imati ovaj tip toka - morate podesiti `Secure SSO`.

Struktura za the `User` object je sljedeća:

[inline-code-attrs-start title = 'Struktura korisnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** Ovo je također id koji se koristi kao userId na objektima komentara. **/
    id: string
    username: string
    /** Link do bloga komentatora, na primjer. **/
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