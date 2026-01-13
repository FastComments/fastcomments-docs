---
Obiekt `Page` reprezentuje stronę, do której może należeć wiele komentarzy. Ten związek jest określony przez `urlId`.

Obiekt `Page` przechowuje informacje takie jak tytuł strony, liczba komentarzy oraz `urlId`.

Struktura obiektu `Page` jest następująca:

[inline-code-attrs-start title = 'Struktura strony'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** Ustawienie tego na null oznacza, że wszyscy użytkownicy SSO mogą zobaczyć stronę. Pusta lista oznacza, że jest zamknięta dla wszystkich użytkowników. **/
    accessibleByGroupIds?: string[] | null
    /** Czy ta strona jest zamknięta dla nowych komentarzy? **/
    isClosed?: boolean
}
[inline-code-end]

---