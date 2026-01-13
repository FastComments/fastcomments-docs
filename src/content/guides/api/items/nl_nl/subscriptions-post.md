[api-resource-header-start name = 'Subscription'; route = 'POST /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Deze API-endpoint maakt het mogelijk een `Subscription` te creëren. Let op dat een gebruiker slechts één subscription per pagina mag hebben, omdat meer overbodig is, en proberen
meer dan één subscription voor dezelfde gebruiker voor dezelfde pagina aan te maken zal resulteren in een fout.

Het aanmaken van een subscription resulteert in het aanmaken van `Notification`-objecten wanneer een nieuwe comment op de root van de geabonneerde `urlId` wordt geplaatst (wanneer comment `parentId` `null` is).

[inline-code-attrs-start title = 'Subscription POST cURL-voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Structuur van Subscription POST-aanvraag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Subscription POST-antwoord'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface SubscriptionPostResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';  
    /** Opgenomen bij mislukking. **/
    reason?: string
}
[inline-code-end]