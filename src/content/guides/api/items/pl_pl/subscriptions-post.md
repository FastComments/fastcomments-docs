[api-resource-header-start name = 'Subscription'; route = 'POST /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Ten endpoint API umożliwia utworzenie `Subscription`. Należy pamiętać, że użytkownik może mieć tylko jedną subskrypcję na stronę, ponieważ więcej jest zbędne, a próba utworzenia więcej niż jednej subskrypcji dla tego samego użytkownika i tej samej strony spowoduje błąd.

Utworzenie subskrypcji spowoduje utworzenie obiektów `Notification`, gdy nowy komentarz zostanie dodany w korzeniu subskrybowanego `urlId` (gdy `parentId` komentarza jest `null`).

[inline-code-attrs-start title = 'Przykład cURL POST subskrypcji'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura żądania POST subskrypcji'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi POST subskrypcji'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface SubscriptionPostResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';  
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
}
[inline-code-end]

---