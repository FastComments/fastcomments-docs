---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| page | number | いいえ |  |

## レスポンス

戻り値: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTagsResponse.ts)

## 例

[inline-code-attrs-start title = 'getHashTags の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-123";
const pageNumber: number = 2;
const responseWithPage: GetHashTagsResponse = await getHashTags(tenantId, pageNumber);
const responseFirstPage: GetHashTagsResponse = await getHashTags(tenantId);
[inline-code-end]

---