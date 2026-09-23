## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| tag | string | Yes |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | No |  |

## Response

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Example

[inline-code-attrs-start title = 'deleteHashTag Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_42";
const tag: string = "high-priority";

const requestBody: DeleteHashTagRequestBody = {
  // populate fields as needed
};

const resultWithBody: APIEmptyResponse = await deleteHashTag(tenantId, tag, requestBody);
const resultWithoutBody: APIEmptyResponse = await deleteHashTag(tenantId, tag);
[inline-code-end]
