## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| yearNumber | number | No |  |
| monthNumber | number | No |  |
| dayNumber | number | No |  |
| skip | number | No |  |

## Odgovor

Returns: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsagesResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getTenantDailyUsages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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