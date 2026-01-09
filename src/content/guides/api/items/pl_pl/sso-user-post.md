[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Ta ścieżka umożliwia utworzenie pojedynczego użytkownika SSO.

Próba utworzenia dwóch użytkowników z tym samym identyfikatorem zakończy się błędem.

[inline-code-attrs-start title = 'Przykład cURL tworzenia SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "my-user-id",
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

W tym przykładzie określamy `groupIds` dla kontroli dostępu, ale jest to opcjonalne.

[inline-code-attrs-start title = 'Struktura żądania tworzenia SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi tworzenia SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    user?: SSOUser; // Zwracamy utworzonego użytkownika w przypadku powodzenia.
}
[inline-code-end]

#### Uwaga integracyjna

Dane przesłane przez API można nadpisać, po prostu przekazując inny ładunek HMAC użytkownika SSO. Na przykład, jeśli ustawisz nazwę użytkownika przez API, ale następnie przekażesz inną w przepływie SSO podczas ładowania strony, automatycznie zaktualizujemy jego nazwę użytkownika.

Nie będziemy aktualizować parametrów użytkownika w tym przepływie, chyba że wyraźnie je określisz lub ustawisz na null (nie undefined).

---