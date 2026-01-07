`User` er et objekt, der repræsenterer en mest-almindelig-fællesnævner for alle brugere.

Husk at hos FastComments har vi en masse forskellige anvendelsestilfælde for brugere:

- Sikker SSO
- Simpel SSO
- Tenant-brugere (For eksempel: Administratorer)
- Kommentatorer

Dette API er til **Kommentatorer** og brugere oprettet via **Simpel SSO**. Grundlæggende kan enhver bruger oprettet
gennem din side tilgås via dette API. Tenant-brugere kan også hentes på denne måde, men du får mere information ved at interagere med `/tenant-users/` API'et.

For `Sikker SSO` brug venligst `/sso-users/` API'et.

Du kan ikke opdatere disse typer brugere. De oprettede deres konto gennem din side, så vi giver nogle grundlæggende skrivebeskyttet adgang, men
du kan ikke foretage ændringer. Hvis du vil have denne type flow - skal du opsætte `Sikker SSO`.

Strukturen for `User`-objektet er som følger:

[inline-code-attrs-start title = 'User Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** This is also the id used as userId on comment objects. **/
    id: string
    username: string
    /** A link to the commenter's blog, for example. **/
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
