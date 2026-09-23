## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Respons

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'deleteQuestionResult Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDelete() {
  const tenantId: string = "tenant_12345";
  const resultId: string = "qr_98765";

  const response: APIEmptyResponse = await deleteQuestionResult(tenantId, resultId);
  console.log(response);
}
[inline-code-end]