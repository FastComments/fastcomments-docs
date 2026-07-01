## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| yearNumber | number | Ne |  |
| monthNumber | number | Ne |  |
| dayNumber | number | Ne |  |
| skip | number | Ne |  |

## Odgovor

Vraća: [`GetTenantDailyUsagesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsagesResponse1.ts)

## Primjer

[inline-code-attrs-start title = 'getTenantDailyUsages Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchDailyUsage() {
  const tenantId: string = "tenant-9876";
  const yearNumber: number = 2024;
  const monthNumber: number = 5; // Maj
  const dayNumber: number = 12;
  const skip: number = 0;

  const fullResult: GetTenantDailyUsagesResponse1 = await getTenantDailyUsages(
    tenantId,
    yearNumber,
    monthNumber,
    dayNumber,
    skip
  );

  // Korištenje samo obaveznog i jednog opcionalnog parametra
  const partialResult: GetTenantDailyUsagesResponse1 = await getTenantDailyUsages(
    tenantId,
    yearNumber
  );

  console.log(fullResult, partialResult);
}

fetchDailyUsage();
[inline-code-end]