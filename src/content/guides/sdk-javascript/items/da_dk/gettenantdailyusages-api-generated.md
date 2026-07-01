## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| yearNumber | number | Nej |  |
| monthNumber | number | Nej |  |
| dayNumber | number | Nej |  |
| skip | number | Nej |  |

## Svar

Returnerer: [`GetTenantDailyUsagesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsagesResponse1.ts)

## Eksempel

[inline-code-attrs-start title = 'getTenantDailyUsages Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

  // Bruger kun påkrævet og en valgfri parameter
  const partialResult: GetTenantDailyUsagesResponse1 = await getTenantDailyUsages(
    tenantId,
    yearNumber
  );

  console.log(fullResult, partialResult);
}

fetchDailyUsage();
[inline-code-end]