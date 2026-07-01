## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| yearNumber | number | Não |  |
| monthNumber | number | Não |  |
| dayNumber | number | Não |  |
| skip | number | Não |  |

## Resposta

Retorna: [`GetTenantDailyUsagesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsagesResponse1.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getTenantDailyUsages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchDailyUsage() {
  const tenantId: string = "tenant-9876";
  const yearNumber: number = 2024;
  const monthNumber: number = 5; // Maio
  const dayNumber: number = 12;
  const skip: number = 0;

  const fullResult: GetTenantDailyUsagesResponse1 = await getTenantDailyUsages(
    tenantId,
    yearNumber,
    monthNumber,
    dayNumber,
    skip
  );

  // Usando apenas o parâmetro obrigatório e um opcional
  const partialResult: GetTenantDailyUsagesResponse1 = await getTenantDailyUsages(
    tenantId,
    yearNumber
  );

  console.log(fullResult, partialResult);
}

fetchDailyUsage();
[inline-code-end]

---