[api-resource-header-start name = 'Page Likes'; route = 'DELETE /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Supprime le like de l'utilisateur actuel d'une page. Si l'utilisateur n'a pas aimé la page, la requête réussit avec le code `not-liked` et ne modifie pas le compteur.

[inline-code-attrs-start title = 'Exemple cURL Page Unlike'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête Page Unlike'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeRequestQueryParams {
    urlId: string
    /** JSON encodé en URI de votre objet SSO. Omettre pour les utilisateurs anonymes. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse Page Unlike'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeResponse {
    status: 'success' | 'failed'
    /** 'not-liked' lorsque l'utilisateur n'avait pas aimé la page. Sinon inclus en cas d'échec. **/
    code?: 'not-liked' | string
    /** Inclus en cas d'échec. **/
    reason?: string
}
[inline-code-end]