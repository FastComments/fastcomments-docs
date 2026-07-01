## Parametre

| Navn | Type | Obligatorisk | Beskrivelse |
|------|------|--------------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Ja |  |

## Svar

Returnerer: [`ReplaceTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReplaceTenantPackageResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'replaceTenantPackage Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-corp-tenant-01";
    const packageId: string = "pkg-2024-annual";

    const config: CustomConfigParameters = {
        // brugerdefinerede konfigurationsfelter her
    };

    const body: ReplaceTenantPackageBody = {
        name: "Enterprise Package",
        // valgfri brugerdefineret konfiguration
        customConfig: config,
    };

    const response: ReplaceTenantPackageResponse = await replaceTenantPackage(tenantId, packageId, body);
    console.log(response);
})();
[inline-code-end]