[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Renvoie le nombre pour chaque réaction sur une page, ainsi que les réactions que l'utilisateur actuel a ajoutées.

[inline-code-attrs-start title = 'Exemple cURL de Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** JSON encodé en URI de votre objet SSO. Omettre pour les utilisateurs anonymes. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Inclus en cas d'échec. **/
    code?: string
    /** Inclus en cas d'échec. **/
    reason?: string
    /** Nombre par identifiant de réaction, par exemple {"heart": 12, "laugh": 3}. Non défini lorsque la page n'a aucune réaction. **/
    counts?: Record<string, number>
    /** Les identifiants de réaction que l'utilisateur effectuant la requête a ajoutés. Non défini lorsqu'ils n'en ont ajouté aucun. **/
    reactedIds?: string[]
}
[inline-code-end]