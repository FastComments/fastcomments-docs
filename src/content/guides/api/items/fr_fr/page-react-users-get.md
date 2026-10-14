[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Renvoie les noms des utilisateurs qui ont ajouté une réaction à une page, triés par ordre alphabétique. Jusqu'à 100 réactions sont recherchées, et les utilisateurs anonymes ne sont pas inclus. Sur les pages qui nécessitent SSO, les utilisateurs qui ne peuvent pas voir la page reçoivent un `403` avec le code `sso-required` ou `page-forbidden`.

Pour obtenir également les avatars et les identifiants d'utilisateur, utilisez [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) avec `includeUserInfo=public`.

[inline-code-attrs-start title = 'Exemple cURL des utilisateurs de réactions de page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête des utilisateurs de réactions de page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** L'identifiant de la réaction. **/
    id: string
    /** JSON encodé en URI de votre objet SSO. Omettre pour les utilisateurs anonymes. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse des utilisateurs de réactions de page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Inclus en cas d'échec. **/
    code?: string
    /** Inclus en cas d'échec. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]