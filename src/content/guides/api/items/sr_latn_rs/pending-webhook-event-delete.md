[api-resource-header-start name = 'PendingWebhookEvent'; route = 'DELETE /api/v1/pending-webhook-events/:id'; creditsCost = 1; api-resource-header-end]

Ovaj endpoint omogućava brisanje jednog `PendingWebhookEvent`.

Ako trebate masovno brisanje, pozovite GET API sa paginacijom, a zatim pozovite ovaj API sekvencijalno.

[inline-code-attrs-start title = 'DELETE PendingWebhookEvent cURL Primer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/pending-webhook-events/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za brisanje PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za brisanje PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventDeleteResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
}
[inline-code-end]

---