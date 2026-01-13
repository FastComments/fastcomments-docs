[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Ta ścieżka umożliwia dodanie pojedynczego autoryzowanego `Vote`. Głosy mogą być `up` (+1) lub `down` (-1).

[inline-code-attrs-start title = 'Przykład cURL: utworzenie Vote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Przykład cURL: utworzenie anonimowego Vote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania tworzenia Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi tworzenia Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### Tworzenie anonimowych głosów

Anonimowe głosy można tworzyć, ustawiając `anonUserId` w parametrach zapytania zamiast `userId`.

To id nie musi odpowiadać żadnemu obiektowi użytkownika (stąd anonimowość). Jest to po prostu identyfikator
sesji, dzięki czemu można ponownie pobrać głosy w tej samej sesji, aby sprawdzić, czy na komentarz zagłosowano.

Jeśli nie masz czegoś takiego jak „anonimowe sesje”, jak robi to FastComments — możesz po prostu
ustawić to na losowe ID, np. UUID (chociaż doceniamy mniejsze identyfikatory, aby oszczędzić miejsce).

### Inne uwagi

- To API przestrzega ustawień na poziomie tenanta. Na przykład, jeśli wyłączysz głosowanie na danej stronie i spróbujesz utworzyć głos przez API, zakończy się to błędem z kodem `voting-disabled`.
- To API jest domyślnie aktywne.
- To API zaktualizuje `votes` odpowiadającego `Comment`.