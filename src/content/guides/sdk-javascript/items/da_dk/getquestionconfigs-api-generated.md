## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| skip | number | Nej |  |

## Svar

Returnerer: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigsResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getQuestionConfigs Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchConfigs() {
  const tenantId: string = "tenant_12345";

  const firstPage: GetQuestionConfigsResponse = await getQuestionConfigs(tenantId);
  const secondPage: GetQuestionConfigsResponse = await getQuestionConfigs(tenantId, 20);
}
[inline-code-end]