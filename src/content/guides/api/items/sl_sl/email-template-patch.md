[api-resource-header-start name = 'EmailTemplate'; route = 'PATCH /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

Ta API endpoint omogoča posodobitev e-poštne predloge z navedbo samo id in atributov, ki jih želite posodobiti.

Upoštevajte, da veljajo enake preveritve kot pri ustvarjanju predloge, na primer:

- Predloga se mora pravilno upodobiti. To se preveri pri vsaki posodobitvi.
- Ne morete imeti podvojenih predlog za isto domeno (v nasprotnem primeru bi bila ena prezreta brez obvestila).

[inline-code-attrs-start title = 'Primer cURL zahteve za EmailTemplate PATCH'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"displayName": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za EmailTemplate PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za EmailTemplate PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePatchResponse {
    status: 'success' | 'failed'
    /** Vključeno ob neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'not-found' | 'unexpected-param' | 'invalid-email-template-id' | 'unauthorized' | 'domain-invalid' | 'duplicate' | 'does-not-render';  
    /** Vključeno ob neuspehu. **/
    reason?: string
    /** Posodobljena e-poštna predloga. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]

---