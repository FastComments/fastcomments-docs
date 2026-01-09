[api-resource-header-start name = 'PendingWebhookEvent'; route = 'GET /api/v1/pending-webhook-events'; creditsCost = 2; api-resource-header-end]

Ta ścieżka zwraca listę oczekujących zdarzeń webhook w parametrze `pendingWebhookEvents`.

To API używa paginacji, udostępnianej przez parametr `skip`. Zdarzenia PendingWebhookEvents są zwracane w stronach po `100`, posortowane według `createdAt` od najnowszych.

[inline-code-attrs-start title = 'Przykład żądania cURL dla PendingWebhookEvent'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pending-webhook-events?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania dla PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsGetQueryParams {
    tenantId: string
    API_KEY: string
    commentId?: string
    externalId?: string
    eventType?: OutboundSyncEventType
    type?: OutboundSyncType
    domain?: string
    /** Zapytanie o zdarzenia z liczbą prób większą niż określona. **/
    attemptCountGT?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi dla PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsGetResponse {
    status: 'success' | 'failed'
    /** Zawarte w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Zawarte w przypadku niepowodzenia. **/
    reason?: string
    pendingWebhookEvents?: PendingWebhookEvent[]
}
[inline-code-end]