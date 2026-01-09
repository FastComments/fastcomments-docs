[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Ta ścieżka umożliwia usunięcie pojedynczego użytkownika SSO na podstawie jego id.

Zwróć uwagę, że ponowne załadowanie widżetu komentarzy z danymi tego użytkownika spowoduje po prostu bezproblemowe ponowne utworzenie użytkownika.

Usunięcie komentarzy użytkownika jest możliwe za pomocą parametru zapytania `deleteComments`. Zauważ, że jeśli ma on wartość true:

1. Wszystkie komentarze użytkownika zostaną usunięte na żywo.
2. Wszystkie __child__ (teraz osierocone) komentarze zostaną usunięte lub zanonimizowane na podstawie konfiguracji strony powiązanej z każdym komentarzem. Na przykład, jeśli tryb usuwania wątku to "anonymize", odpowiedzi pozostaną, a komentarze użytkownika zostaną zanonimizowane. Dotyczy to tylko, gdy `commentDeleteMode` jest ustawione na `Remove` (wartość domyślna).
3. Wartość `creditsCost` staje się `2`.

### Zanonimizowane komentarze

Możesz zachować komentarze użytkownika, ale po prostu je zanonimizować ustawiając `commentDeleteMode=1`.

Jeśli komentarze użytkownika zostaną zanonimizowane, następujące wartości zostaną ustawione na null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

Właściwości `isDeleted` i `isDeletedUser` są ustawiane na `true`.

Podczas renderowania widżet komentarzy użyje `DELETED_USER_PLACEHOLDER` (domyślnie: "[deleted]") dla nazwy użytkownika oraz `DELETED_CONTENT_PLACEHOLDER` dla treści komentarza. Można to dostosować za pomocą interfejsu dostosowywania widżetu (Widget Customization UI).

### Przykłady

[inline-code-attrs-start title = 'Przykład cURL usunięcia SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania usunięcia SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // domyślnie
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Możesz ustawić to na true, aby również usunąć komentarze użytkownika. Spowoduje to podwojenie kosztu kredytów. **/
    deleteComments?: 'true' | 'false'
    /** Możesz ustawić to według potrzeb, aby określić, jak obsłużyć komentarze użytkownika. **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi usunięcia SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** Dołączane w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** Dołączane w przypadku niepowodzenia. **/
    reason?: string
    user?: SSOUser; // Zwracamy usuniętego użytkownika w przypadku powodzenia.
}
[inline-code-end]