[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

Ta ruta omogoča pošiljanje prijavne povezave enemu `TenantUser`.

Uporabno pri množičnem ustvarjanju uporabnikov, ko jim ni treba navodila za prijavo na FastComments.com. To jim pošlje le "čarobno povezavo" za prijavo, ki poteče po `30 days`.

Naslednje omejitve veljajo za pošiljanje prijavne povezave `TenantUser`:
- `TenantUser` mora že obstajati.
- Morate imeti dostop do upravljanja `Tenant`, kateremu pripada `TenantUser`.

Prijavno povezavo `TenantUser` lahko pošljemo na naslednji način:

[inline-code-attrs-start title = 'Primer cURL zahtevka za prijavno povezavo TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

To bo poslalo e-pošto, kot na primer `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Struktura zahteve za prijavno povezavo TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za prijavno povezavo TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Vključeno ob napaki. **/
    reason?: string
}
[inline-code-end]