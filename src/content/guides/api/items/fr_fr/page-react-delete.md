[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Supprime l'une des réactions de l'utilisateur actuel d'une page. Si l'utilisateur n'a pas ajouté cette réaction, la requête réussit avec le code `no-react` et ne modifie pas le compteur.

[inline-code-attrs-start title = 'Exemple cURL de suppression de réaction de page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête de suppression de réaction de page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** L'identifiant de la réaction. **/
    id: string
    /** JSON encodé en URI de votre objet SSO. Omettre pour les utilisateurs anonymes. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse de suppression de réaction de page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react' lorsque l'utilisateur n'avait pas ajouté cette réaction. Sinon inclus en cas d'échec. **/
    code?: 'no-react' | string
    /** Inclus en cas d'échec. **/
    reason?: string
}
[inline-code-end]