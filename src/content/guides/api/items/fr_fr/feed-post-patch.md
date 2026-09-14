[api-resource-header-start name = 'FeedPost'; route = 'PATCH /api/v1/feed-posts/:id'; creditsCost = 1; api-resource-header-end]

Cette route met à jour un seul `FeedPost`. Envoyez uniquement les champs que vous souhaitez modifier.

[inline-code-attrs-start title = 'Exemple cURL de mise à jour FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/feed-posts/some-post-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "title": "Release 2.0.1 is out",
    "tags": ["releases", "hotfix"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête de mise à jour FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPatchQueryParams {
    tenantId: string
    API_KEY: string
}

/** Tout champ modifiable de la structure FeedPost. **/
type FeedPostPatchBody = Partial<Pick<FeedPost, 'title' | 'contentHTML' | 'fromUserDisplayName' | 'tags' | 'weight' | 'meta' | 'media' | 'links'>>
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse de mise à jour FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPatchResponse {
    status: 'success' | 'failed'
    /** Inclus en cas d'échec. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'internal'
    /** Inclus en cas d'échec. **/
    reason?: string
}
[inline-code-end]