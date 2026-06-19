## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| page | number | 否 |  |

## 回應

回傳: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTagsResponse.ts)

## 範例

[inline-code-attrs-start title = 'getHashTags 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-123";
const pageNumber: number = 2;
const responseWithPage: GetHashTagsResponse = await getHashTags(tenantId, pageNumber);
const responseFirstPage: GetHashTagsResponse = await getHashTags(tenantId);
[inline-code-end]

---