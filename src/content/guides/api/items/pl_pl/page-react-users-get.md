[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Zwraca nazwy użytkowników, którzy dodali reakcję do strony, posortowane alfabetycznie. Wyszukiwane jest maksymalnie 100 reakcji, a anonimowi użytkownicy oraz użytkownicy z prywatną aktywnością profilu nie są uwzględniani. Na stronach wymagających SSO, użytkownicy, którzy nie mogą wyświetlić strony, otrzymują `403` z kodem `sso-required` lub `page-forbidden`.

Aby również uzyskać awatary i identyfikatory użytkowników, użyj [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) z `includeUserInfo=public`.

[inline-code-attrs-start title = 'Przykład cURL dla Page React Users'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania Page React Users'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** Id reakcji. **/
    id: string
    /** Zakodowany w URI JSON twojego obiektu SSO. Pomiń dla anonimowych użytkowników. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi Page React Users'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Zawarte w przypadku niepowodzenia. **/
    code?: string
    /** Zawarte w przypadku niepowodzenia. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]