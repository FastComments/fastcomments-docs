## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| page | number | いいえ |  |

## レスポンス

戻り値: [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTags200Response.ts)

## 例

[inline-code-attrs-start title = 'getHashTags の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f4b2c3a';
const tagsFirstPage: GetHashTags200Response = await getHashTags(tenantId);
const tagsSecondPage: GetHashTags200Response = await getHashTags(tenantId, 2);
console.log(tagsFirstPage, tagsSecondPage);
[inline-code-end]

---