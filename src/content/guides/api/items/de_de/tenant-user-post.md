[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Diese Route bietet die Möglichkeit, einen einzelnen `TenantUser` hinzuzufügen.

Das Erstellen eines `TenantUser` hat die folgenden Einschränkungen:

- Ein `username` ist erforderlich.
- Eine `email` ist erforderlich.
- Das `signUpDate` darf nicht in der Zukunft liegen.
- Die `locale` muss in der Liste der [Unterstützten Locales](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1) enthalten sein.
- Der `username` muss über alle FastComments.com hinweg eindeutig sein. Wenn dies ein Problem ist, empfehlen wir stattdessen SSO zu verwenden.
- Die `email` muss über alle FastComments.com hinweg eindeutig sein. Wenn dies ein Problem ist, empfehlen wir stattdessen SSO zu verwenden.
- Sie können nicht mehr Tenant-Benutzer erstellen als unter `maxTenantUsers` in Ihrem Paket definiert.

Wir können einen `TenantUser` wie folgt erstellen

[inline-code-attrs-start title = 'TenantUser Erstellen cURL Beispiel'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Erstellen Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Erstellen Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Included on failure. **/
    reason?: string
    tenantUser?: TenantUser; // We return the complete created tenant user on success.
}
[inline-code-end]
