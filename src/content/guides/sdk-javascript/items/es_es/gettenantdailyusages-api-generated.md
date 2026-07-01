## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| yearNumber | number | No |  |
| monthNumber | number | No |  |
| dayNumber | number | No |  |
| skip | number | No |  |

## Respuesta

Devuelve: [`GetTenantDailyUsagesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsagesResponse1.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getTenantDailyUsages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchDailyUsage() {
  const tenantId: string = "tenant-9876";
  const yearNumber: number = 2024;
  const monthNumber: number = 5; // Mayo
  const dayNumber: number = 12;
  const skip: number = 0;

  const fullResult: GetTenantDailyUsagesResponse1 = await getTenantDailyUsages(
    tenantId,
    yearNumber,
    monthNumber,
    dayNumber,
    skip
  );

  // Usando solo el parámetro requerido y uno opcional
  const partialResult: GetTenantDailyUsagesResponse1 = await getTenantDailyUsages(
    tenantId,
    yearNumber
  );

  console.log(fullResult, partialResult);
}

fetchDailyUsage();
[inline-code-end]