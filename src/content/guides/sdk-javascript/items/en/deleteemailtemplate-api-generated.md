## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Example

[inline-code-attrs-start title = 'deleteEmailTemplate Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface APIStatus {
  code: number;
  message: string;
}

interface APIEmptyResponse {
  status?: APIStatus;
}

(async () => {
  const tenantId: string = "tenant_12345";
  const templateId: string = "template_987";

  const result: APIEmptyResponse = await deleteEmailTemplate(tenantId, templateId);
})();
[inline-code-end]
