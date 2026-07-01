## Parametri

| Naziv | Tip | Obavezan | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| id | string | Da |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Da |  |

## Odgovor

Vraća: [`ReplaceTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReplaceTenantPackageResponse.ts)

## Primjer

[inline-code-attrs-start title = 'replaceTenantPackage Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-corp-tenant-01";
    const packageId: string = "pkg-2024-annual";

    const config: CustomConfigParameters = {
        // prilagođena polja konfiguracije ovdje
    };

    const body: ReplaceTenantPackageBody = {
        name: "Enterprise Package",
        // opcionalna prilagođena konfiguracija
        customConfig: config,
    };

    const response: ReplaceTenantPackageResponse = await replaceTenantPackage(tenantId, packageId, body);
    console.log(response);
})();
[inline-code-end]