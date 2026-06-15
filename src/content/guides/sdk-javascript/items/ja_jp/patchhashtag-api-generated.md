---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tag | string | はい |  |
| tenantId | string | いいえ |  |
| updateHashTagBody | UpdateHashTagBody | いいえ |  |

## レスポンス

返却: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchHashTag200Response.ts)

## 例

[inline-code-attrs-start title = 'patchHashTag の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "feature-request";
const tenantId: string = "tenant_8f7a3b2c";
const updateHashTagBody: UpdateHashTagBody = {
  displayName: "Feature Request",
  description: "Use this tag for requests to add new features to the product",
  enabled: true
};
const result: PatchHashTag200Response = await patchHashTag(tag, tenantId, updateHashTagBody);
[inline-code-end]

---