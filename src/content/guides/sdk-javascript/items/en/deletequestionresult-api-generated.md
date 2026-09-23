## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Example

[inline-code-attrs-start title = 'deleteQuestionResult Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDelete() {
  const tenantId: string = "tenant_12345";
  const resultId: string = "qr_98765";

  const response: APIEmptyResponse = await deleteQuestionResult(tenantId, resultId);
  console.log(response);
}
[inline-code-end]
