[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Diese Route bietet die Möglichkeit, einen einzelnen `TenantUser` zu aktualisieren.

Das Aktualisieren eines `TenantUser` hat die folgenden Einschränkungen:

- Das `signUpDate` darf nicht in der Zukunft liegen.
- Die `locale` muss in der Liste der [Unterstützten Locales](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1) enthalten sein.
- Der `username` muss über alle FastComments.com hinweg eindeutig sein. Wenn dies ein Problem ist, empfehlen wir stattdessen SSO zu verwenden.
- Die `email` muss über alle FastComments.com hinweg eindeutig sein. Wenn dies ein Problem ist, empfehlen wir stattdessen SSO zu verwenden.
- Sie können die `tenantId` eines Benutzers nicht aktualisieren.

Wir können einen `TenantUser` wie folgt erstellen

[inline-code-attrs-start title = 'TenantUser Aktualisieren cURL Beispiel'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Aktualisieren Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** When email or username is changed, you can set this to true to also update the user's comments. This will double the credit cost. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Aktualisieren Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
