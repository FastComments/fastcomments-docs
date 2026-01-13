`User` to obiekt, który reprezentuje wspólny mianownik wszystkich użytkowników.

Pamiętaj, że w FastComments mamy wiele różnych zastosowań użytkowników:

- Secure SSO
- Simple SSO
- Tenant Users (Na przykład: Administratorzy)
- Commenters

To API jest przeznaczone dla **Commenters** oraz użytkowników utworzonych przez **Simple SSO**. Zasadniczo każdy użytkownik utworzony przez Twoją stronę może być dostępny przez to API. Tenant Users można również pobrać w ten sposób, ale uzyskasz więcej informacji, korzystając z API `/tenant-users/`.

Dla `Secure SSO` użyj API `/sso-users/`.

Nie możesz aktualizować tych typów użytkowników. Utworzyli oni swoje konto przez Twoją stronę, więc zapewniamy podstawowy dostęp tylko do odczytu, ale nie możesz wprowadzać zmian. Jeśli chcesz mieć taki sposób działania — musisz skonfigurować `Secure SSO`.

Struktura obiektu `User` jest następująca:

[inline-code-attrs-start title = 'Struktura obiektu User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** To jest również id używane jako userId w obiektach komentarzy. **/
    id: string
    username: string
    /** Na przykład link do bloga komentującego. **/
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