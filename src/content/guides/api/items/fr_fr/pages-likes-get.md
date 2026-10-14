[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

Renvoie le nombre de likes sur une page, et éventuellement les utilisateurs qui l'ont likée, les plus récents en premier. Il s'agit de la version côté serveur de [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get).

`includeUserInfo` contrôle quels utilisateurs sont retournés :

- `public` exclut les utilisateurs qui ont rendu leurs activités de profil privées.
- `include-private` les inclut. N'utilisez cela que lorsque la liste n'est pas affichée au public.

Les utilisateurs anonymes ne sont jamais inclus, et jusqu'à 100 likes sont recherchés. Les pages qui n'existent pas renvoient un `likeCount` de `0` et une liste `users` vide.

[inline-code-attrs-start title = 'Exemple cURL des likes de page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête des likes de page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Omettre pour ne récupérer que le nombre de likes. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse des likes de page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Inclus en cas d'échec. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Inclus en cas d'échec. **/
    reason?: string
    likeCount?: number
    /** Les plus récents en premier. Défini uniquement lorsque includeUserInfo est fourni. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Astuce utile

N'oubliez pas d'encoder en URI les valeurs comme le `urlId`.