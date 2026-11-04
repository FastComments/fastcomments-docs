[api-resource-header-start name = 'PendingWebhookEvent'; route = 'GET /api/v1/pending-webhook-events/count'; creditsCost = 2; api-resource-header-end]

Ta trasa zwraca obiekt zawierający liczbę oczekujących zdarzeń webhook w parametrze `count`.

Możesz filtrować według tych samych parametrów co endpoint `/pending-webhook-events`

[inline-code-attrs-start title = 'Przykład cURL dla PendingWebhookEvent Count'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pending-webhook-events/count?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania PendingWebhookEvent Count'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsCountQueryParams {
    tenantId: string
    API_KEY: string
    commentId?: string
    externalId?: string
    eventType?: OutboundSyncEventType
    type?: OutboundSyncType
    domain?: string
    /** Zapytanie o zdarzenia z liczbą prób większą niż podana liczba. **/
    attemptCountGT?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi PendingWebhookEvent Count'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsCountResponse {
    status: 'success' | 'failed'
    /** Dołączane w przypadku błędu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Dołączane w przypadku błędu. **/
    reason?: string
    count?: number
}
[inline-code-end]