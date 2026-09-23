[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Dodaje reakcję do strony jako bieżący użytkownik. Użytkownik może dodać każdy identyfikator reakcji tylko raz: ponowne dodanie zakończy się sukcesem z kodem `already-reacted` i nie zmieni licznika. Użytkownik może dodać kilka różnych reakcji do tej samej strony.

Identyfikatory reakcji są wybierane przez Ciebie i mogą mieć do 36 znaków. Strona zostanie utworzona, jeśli jeszcze nie istnieje. Przekaż `title`, aby ustawić lub zaktualizować tytuł strony.

[inline-code-attrs-start title = 'Przykład cURL reakcji na stronie'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania reakcji na stronie'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** The reaction id, up to 36 characters. **/
    id: string
    /** Sets the page's title. **/
    title?: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi reakcji na stronie'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** 'already-reacted' when the user had already added this reaction. 'react-id-too-long' (HTTP 422) when the id is over 36 characters. Otherwise included on failure. **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]