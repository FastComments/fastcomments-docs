`User` je objekt, ki predstavlja najpogostejši skupni imenovalec vseh uporabnikov.

Upoštevajte, da imamo pri FastComments več različnih primerov uporabe uporabnikov:

- Secure SSO
- Simple SSO
- Uporabniki najemnika (na primer: skrbniki)
- Komentatorji

Ta API je namenjen **Komentatorjem** in uporabnikom, ustvarjenim z **Simple SSO**. Praktično vsak uporabnik, ustvarjen prek vaše strani, je dostopen preko tega API-ja. Uporabniki najemnika so prav tako dostopni na ta način, vendar boste več informacij dobili pri interakciji z API-jem `/tenant-users/`.

Za `Secure SSO` uporabite API `/sso-users/`.

Te vrste uporabnikov ne morete posodobiti. Ustvarili so svoj račun preko vaše strani, zato nudimo osnovni dostop samo za branje, vendar ne morete narediti sprememb. Če želite imeti takšen potek, morate nastaviti `Secure SSO`.

Struktura objekta `User` je naslednja:

[inline-code-attrs-start title = 'Struktura uporabnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** To je tudi id, uporabljen kot userId na objektih komentarjev. **/
    id: string
    username: string
    /** Na primer, povezava do bloga komentatorja. **/
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

---