---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Yanıt

Döndürür: [`GetTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackageResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getTenantPackage Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
function fetchPackage(tenantId?: string, id?: string): Promise<GetTenantPackageResponse> {
    const tid: string = tenantId ?? "acme-corp-001";
    const pid: string = id ?? "premium-plan-2024";
    return getTenantPackage(tid, pid);
}

async function runExample(): Promise<void> {
    const response: GetTenantPackageResponse = await fetchPackage();
    const status: APIStatus | undefined = response.status;
    const pkg: TenantPackage | undefined = response.package;
}
[inline-code-end]

---