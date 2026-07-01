## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Yes |  |

## Odpowiedź

Zwraca: [`ReplaceTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReplaceTenantPackageResponse.ts)

## Przykład

[inline-code-attrs-start title = 'replaceTenantPackage Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-corp-tenant-01";
    const packageId: string = "pkg-2024-annual";

    const config: CustomConfigParameters = {
        // własne pola konfiguracyjne tutaj
    };

    const body: ReplaceTenantPackageBody = {
        name: "Enterprise Package",
        // opcjonalna niestandardowa konfiguracja
        customConfig: config,
    };

    const response: ReplaceTenantPackageResponse = await replaceTenantPackage(tenantId, packageId, body);
    console.log(response);
})();
[inline-code-end]