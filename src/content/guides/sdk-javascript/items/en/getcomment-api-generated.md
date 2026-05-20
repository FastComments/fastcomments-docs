## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComment200Response.ts)

## Example

[inline-code-attrs-start title = 'getComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-84a2f6";
const id: string = "cmt-9f2b7e4a";
const includeReplies: boolean | undefined = undefined;
const result: GetComment200Response = await getComment(tenantId, id);
[inline-code-end]
