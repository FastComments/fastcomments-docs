[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Renvoie les noms des utilisateurs qui ont ajouté une réaction à une page, triés par ordre alphabétique. Jusqu'à 100 réactions sont recherchées, et les utilisateurs anonymes ne sont pas inclus.

[inline-code-attrs-start title = 'Exemple cURL des utilisateurs de réactions de page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête des utilisateurs de réactions de page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** The reaction id. **/
    id: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse des utilisateurs de réactions de page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: string
    /** Included on failure. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]