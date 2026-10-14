[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Renvoie le nombre pour chaque réaction sur une page, ainsi que les réactions que l'utilisateur actuel a ajoutées.

Passez `includeUserInfo=public` pour également obtenir les utilisateurs qui ont réagi, sous forme d'une entrée par réaction avec son identifiant de réaction, la plus récente en premier. Les utilisateurs avec une activité de profil privé et les utilisateurs anonymes sont exclus, et jusqu'à 100 réactions sont recherchées. Seul `public` est accepté ici ; `include-private` renvoie un `422` et est disponible depuis [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) avec une clé API. Sur les pages qui nécessitent le SSO, `includeUserInfo` renvoie un `403` pour les utilisateurs qui ne peuvent pas voir la page.

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
    /** Définir à 'public' pour inclure les utilisateurs qui ont réagi. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Inclus en cas d'échec. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Inclus en cas d'échec. **/
    reason?: string
    /** Nombre par identifiant de réaction, par exemple {"heart": 12, "laugh": 3}. Non défini lorsque la page n'a aucune réaction. **/
    counts?: Record<string, number>
    /** Les identifiants de réaction que l'utilisateur effectuant la requête a ajoutés. Non défini lorsqu'il n'en a ajouté aucun. **/
    reactedIds?: string[]
    /** Une entrée par réaction, la plus récente en premier. Défini uniquement lorsque includeUserInfo est fourni. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** L'identifiant de réaction, par exemple "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

---