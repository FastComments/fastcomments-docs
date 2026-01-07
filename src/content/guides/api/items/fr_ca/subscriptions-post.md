[api-resource-header-start name = 'Subscription'; route = 'POST /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Ce endpoint API fournit la capacité de créer une `Subscription`. Notez qu'un utilisateur ne peut avoir qu'un seul abonnement par page, car plus est redondant, et essayer
de créer plus d'un abonnement pour le même utilisateur pour la même page résultera en une erreur.

Créer un abonnement résultera en la création d'objets `Notification` lorsqu'un nouveau commentaire est laissé à la racine du `urlId` abonné (lorsque le `parentId` du commentaire est `null`).

[inline-code-attrs-start title = 'Exemple cURL de POST de Subscription'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "userId": "tenantId:test-user-id",
    "urlId": "some-page-id-or-url",
    "url": "https://example.com/page",
    "pageTitle": "Some Example Page!"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête POST de Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse POST de Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface SubscriptionPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
