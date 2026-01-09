[api-resource-header-start name = 'TenantPackage'; route = 'DELETE /api/v1/tenant-packages/:id'; creditsCost = 5; api-resource-header-end]

Ova ruta omogućava uklanjanje `TenantPackage` po id-u.

Ne možete ukloniti `TenantPackage` koji je u upotrebi (tenant-ov `packageId` pokazuje na paket). Prvo ažurirajte `Tenant`.

[inline-code-attrs-start title = 'Primer cURL zahteva za uklanjanje TenantPackage-a'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za uklanjanje TenantPackage-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackageDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za uklanjanje TenantPackage-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackageDeleteResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized' | 'package-in-use'
    /** Uključeno u slučaju greške. **/
    reason?: string
}
[inline-code-end]

---