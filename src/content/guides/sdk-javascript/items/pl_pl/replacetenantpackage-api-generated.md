---
## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Tak |  |

## Odpowiedź

Zwraca: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Przykład

[inline-code-attrs-start title = 'replaceTenantPackage Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function replacePackageDemo(): Promise<void> {
  const tenantId: string = "acme-corp-001";
  const packageId: string = "basic-plan-2023";
  const replaceBody: ReplaceTenantPackageBody = {
    newPackageId: "enterprise-plan-2024"
    // opcjonalne pola mogą być dodane tutaj w razie potrzeby
  };
  const response: APIEmptyResponse = await replaceTenantPackage(tenantId, packageId, replaceBody);
  console.log(response);
}
[inline-code-end]

---