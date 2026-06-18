## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| page | number | 否 |  |
| limit | number | 否 |  |
| skip | number | 否 |  |
| asTree | boolean | 否 |  |
| skipChildren | number | 否 |  |
| limitChildren | number | 否 |  |
| maxTreeDepth | number | 否 |  |
| urlId | string | 否 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |
| contextUserId | string | 否 |  |
| hashTag | string | 否 |  |
| parentId | string | 否 |  |
| direction | SortDirections | 否 |  |
| fromDate | number | 否 |  |
| toDate | number | 否 |  |

## 回應

回傳：[`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## 範例

[inline-code-attrs-start title = 'getComments 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a12b3";
const response: GetComments200Response = await getComments(tenantId, 1, 20, 0, true, 0, 3, 2, "https://mysite.com/posts/678", undefined, undefined, undefined, undefined, "parent_987", undefined, 1716873600000, 1719552000000);
[inline-code-end]

---