## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| skip | number | Nej |  |

## Svar

Returnerer: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigs200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'getQuestionConfigs Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-42a7b9';
const firstPage: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId);
const secondPage: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId, 50);
[inline-code-end]

---