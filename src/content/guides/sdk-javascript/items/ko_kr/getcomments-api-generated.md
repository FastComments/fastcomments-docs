## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| page | number | 아니오 |  |
| limit | number | 아니오 |  |
| skip | number | 아니오 |  |
| asTree | boolean | 아니오 |  |
| skipChildren | number | 아니오 |  |
| limitChildren | number | 아니오 |  |
| maxTreeDepth | number | 아니오 |  |
| urlId | string | 아니오 |  |
| userId | string | 아니오 |  |
| anonUserId | string | 아니오 |  |
| contextUserId | string | 아니오 |  |
| hashTag | string | 아니오 |  |
| parentId | string | 아니오 |  |
| direction | SortDirections | 아니오 |  |
| fromDate | number | 아니오 |  |
| toDate | number | 아니오 |  |

## 응답

반환: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Example

[inline-code-attrs-start title = 'getComments 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a12b3";
const response: GetComments200Response = await getComments(tenantId, 1, 20, 0, true, 0, 3, 2, "https://mysite.com/posts/678", undefined, undefined, undefined, undefined, "parent_987", undefined, 1716873600000, 1719552000000);
[inline-code-end]