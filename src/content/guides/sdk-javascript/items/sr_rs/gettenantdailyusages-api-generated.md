## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| yearNumber | number | Не |  |
| monthNumber | number | Не |  |
| dayNumber | number | Не |  |
| skip | number | Не |  |

## Одговор

Враћа: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsagesResponse.ts)

## Пример

[inline-code-attrs-start title = 'getTenantDailyUsages Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetTenantDailyUsages() {
  const tenantId: string = "tenant_9876";
  const yearNumber: number = 2024;
  const monthNumber: number = 2;
  const dayNumber: number = 28;
  const skip: number = 10;

  const fullResult: GetTenantDailyUsagesResponse = await getTenantDailyUsages(
    tenantId,
    yearNumber,
    monthNumber,
    dayNumber,
    skip
  );

  const partialResult: GetTenantDailyUsagesResponse = await getTenantDailyUsages(
    tenantId,
    yearNumber
  );
}
[inline-code-end]

---