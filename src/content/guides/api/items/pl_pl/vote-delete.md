[api-resource-header-start name = 'Vote'; route = 'DELETE /api/v1/votes/:id'; creditsCost = 1; api-resource-header-end]

Ta ścieżka umożliwia usunięcie pojedynczego `Vote`.

[inline-code-attrs-start title = 'Przykład żądania cURL usunięcia głosu'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/votes/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania usunięcia głosu'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi usunięcia głosu'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
}
[inline-code-end]

Notes:

- To API przestrzega ustawień na poziomie najemcy. Na przykład, jeśli wyłączysz głosowanie dla danej strony, i spróbujesz utworzyć głos przez API, operacja zakończy się niepowodzeniem z kodem błędu `voting-disabled`.
- To API jest domyślnie aktywne.
- To API zaktualizuje `votes` odpowiadającego `Comment`.