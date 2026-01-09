[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

Ta trasa umożliwia usunięcie `TenantUser` po id.

Usunięcie komentarzy użytkownika jest możliwe za pomocą parametru zapytania `deleteComments`. Zauważ, że jeśli to jest true:

1. Wszystkie komentarze użytkownika zostaną usunięte na żywo.
2. Wszystkie __child__ (teraz osierocone) komentarze zostaną usunięte lub zanonimizowane w zależności od konfiguracji strony powiązanej z każdym komentarzem. Na przykład jeśli tryb usuwania wątku to "anonymize", odpowiedzi pozostaną, a komentarze użytkownika zostaną zanonimizowane. Dotyczy to tylko gdy `commentDeleteMode` ma wartość `Remove` (wartość domyślna).
3. `creditsCost` zmienia się na `2`.

### Zanonimizowane komentarze

Możesz zachować komentarze użytkownika, ale zanonimizować je, ustawiając `commentDeleteMode=1`.

Jeśli komentarze użytkownika zostaną zanonimizowane, następujące wartości zostaną ustawione na null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` i `isDeletedUser` zostają ustawione na `true`.

Podczas renderowania widget komentarzy użyje `DELETED_USER_PLACEHOLDER` (domyślnie: "[deleted]") jako nazwy użytkownika oraz `DELETED_CONTENT_PLACEHOLDER` dla treści komentarza. Można je dostosować za pomocą interfejsu Widget Customization UI.

### Przykłady

[inline-code-attrs-start title = 'Przykład cURL usuwania TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania usunięcia TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // domyślnie
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Możesz ustawić to na true, aby również usunąć komentarze użytkownika. Spowoduje to podwojenie kosztu kredytów. **/
    deleteComments?: 'true' | 'false'
    /** Możesz ustawić to zgodnie z potrzebą, aby określić, jak obsługiwać komentarze użytkownika. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi usunięcia TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Zawarte w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Zawarte w przypadku niepowodzenia. **/
    reason?: string
}
[inline-code-end]