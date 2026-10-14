[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

Renvoie le nombre pour chaque réaction sur une page, et éventuellement les utilisateurs qui ont réagi, sous forme d'une entrée par réaction avec son identifiant de réaction, la plus récente en premier. Il s'agit de la version côté serveur de [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get).

`includeUserInfo` contrôle quels utilisateurs sont renvoyés :

- `public` exclut les utilisateurs qui ont rendu privées leurs activités de profil.
- `include-private` les inclut. N'utilisez cela que lorsque la liste n'est pas affichée au public.

Les utilisateurs anonymes ne sont jamais inclus, et jusqu'à 100 réactions sont recherchées. Les pages qui n'existent pas renvoient aucun `counts` et une liste `users` vide.

[inline-code-attrs-start title = 'Exemple cURL de réactions de page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Omettre pour ne récupérer que les comptes. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Inclus en cas d'échec. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Inclus en cas d'échec. **/
    reason?: string
    /** Nombre par identifiant de réaction, par exemple {"heart": 12, "laugh": 3}. Non défini lorsque la page n'a aucune réaction. **/
    counts?: Record<string, number>
    /** Une entrée par réaction, la plus récente en premier. Défini uniquement lorsque includeUserInfo est fourni. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** L'identifiant de la réaction, par exemple "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Astuce utile

N'oubliez pas d'encoder en URI les valeurs comme le `urlId`.

---