## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tag | string | 是 |  |
| tenantId | string | 否 |  |
| updateHashTagBody | UpdateHashTagBody | 否 |  |

## 回應

回傳: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## 範例

[inline-code-attrs-start title = 'patchHashTag 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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