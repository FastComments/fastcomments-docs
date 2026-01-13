[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Diese Route ermöglicht die Erstellung eines einzelnen SSO-Benutzers.

Der Versuch, zwei Benutzer mit derselben ID zu erstellen, führt zu einem Fehler.

[inline-code-attrs-start title = 'SSOUser Erstellen cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

In diesem Beispiel geben wir `groupIds` für die Zugriffskontrolle an, aber dies ist optional.

[inline-code-attrs-start title = 'SSOUser Erstellen Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser Erstellen Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Included on failure. **/
    reason?: string
    user?: SSOUser; // We return the created user on success.
}
[inline-code-end]

#### Integrationshinweis

Daten, die über die API übergeben werden, können einfach überschrieben werden, indem ein anderer SSO-Benutzer-HMAC-Payload übergeben wird. Wenn Sie beispielsweise
einen Benutzernamen über die API setzen, aber dann einen anderen über den SSO-Flow beim Laden der Seite übergeben, aktualisieren wir automatisch
ihren Benutzernamen.

Wir aktualisieren keine Benutzerparameter in diesem Flow, es sei denn, Sie geben sie explizit an oder setzen sie auf null (nicht undefined).
