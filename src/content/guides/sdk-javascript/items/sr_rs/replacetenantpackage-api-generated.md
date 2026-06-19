---
## Parametri

| Naziv | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Да |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Пример

[inline-code-attrs-start title = 'Primer replaceTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b1c';
const id: string = 'pkg_pro_2026';
const replaceTenantPackageBody: ReplaceTenantPackageBody = {
  planCode: 'pro_annual',
  seats: 12,
  expiresAt: '2027-01-01T00:00:00Z',
  autoRenew: true, // opciona zastavica koja demonstrira opcioni parametar
  notes: 'Upgrade for team collaboration'
};
const result: APIEmptyResponse = await replaceTenantPackage(tenantId, id, replaceTenantPackageBody);
[inline-code-end]

---