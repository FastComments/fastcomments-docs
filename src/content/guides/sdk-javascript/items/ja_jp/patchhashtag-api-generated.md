## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tag | string | Yes |  |
| tenantId | string | No |  |
| updateHashTagBody | UpdateHashTagBody | No |  |

## レスポンス

戻り値: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## 例

[inline-code-attrs-start title = 'patchHashTag の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "release-notes";
const tenantId: string = "tenant_8421";
const updateHashTagBody: UpdateHashTagBody = {
  name: "Release Notes",
  description: "Thread for discussing feature releases and changelogs",
  isActive: true
};
const result: UpdateHashTagResponse = await patchHashTag(tag, tenantId, updateHashTagBody);
[inline-code-end]

---