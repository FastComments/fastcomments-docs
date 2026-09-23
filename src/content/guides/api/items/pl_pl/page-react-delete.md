[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Usuwa jedną z reakcji bieżącego użytkownika ze strony. Jeśli użytkownik nie dodał tej reakcji, żądanie kończy się sukcesem z kodem `no-react` i nie zmienia licznika.

[inline-code-attrs-start title = 'Przykład cURL usuwania reakcji na stronie'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania usuwania reakcji na stronie'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** Identyfikator reakcji. **/
    id: string
    /** Zakodowany w URI JSON twojego obiektu SSO. Pomiń dla anonimowych użytkowników. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi usuwania reakcji na stronie'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react' gdy użytkownik nie dodał tej reakcji. W przeciwnym razie zawarte przy niepowodzeniu. **/
    code?: 'no-react' | string
    /** Zawarte przy niepowodzeniu. **/
    reason?: string
}
[inline-code-end]

---