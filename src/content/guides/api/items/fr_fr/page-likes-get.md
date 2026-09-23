[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Renvoie le nombre de likes sur une page, ainsi que le fait que l'utilisateur actuel l'ait aimée. Les pages qui n'existent pas encore renvoient un `likeCount` de `0`.

[inline-code-attrs-start title = 'Exemple cURL des likes de page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête des likes de page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** JSON encodé en URI de votre objet SSO. Omettre pour les utilisateurs anonymes. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse des likes de page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Inclus en cas d'échec. **/
    code?: string
    /** Inclus en cas d'échec. **/
    reason?: string
    likeCount: number
    /** Indique si l'utilisateur effectuant la requête a aimé la page. **/
    didLike: boolean
    /** Le nombre de commentaires de premier niveau sur la page. **/
    commentCount: number
    /** L'identifiant utilisé pour s'abonner aux mises à jour en direct de cette page. **/
    urlIdWS: string
}
[inline-code-end]