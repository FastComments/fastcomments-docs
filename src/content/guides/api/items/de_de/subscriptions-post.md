[api-resource-header-start name = 'Subscription'; route = 'POST /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Dieser API-Endpunkt bietet die Möglichkeit, ein `Subscription` zu erstellen. Beachten Sie, dass ein Benutzer nur ein Abonnement pro Seite haben kann, da mehr redundant ist, und der Versuch,
mehr als ein Abonnement für denselben Benutzer für dieselbe Seite zu erstellen, führt zu einem Fehler.

Das Erstellen eines Abonnements führt dazu, dass `Notification`-Objekte erstellt werden, wenn ein neuer Kommentar am Wurzelknoten der abonnierten `urlId` hinterlassen wird (wenn `parentId` des Kommentars `null` ist).

[inline-code-attrs-start title = 'Abonnement POST cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Abonnement POST Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Abonnement POST Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface SubscriptionPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
