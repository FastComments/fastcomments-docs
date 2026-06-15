---
## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | string | 是 |  |
| tenantId | string | 否 |  |
| updateHashTagBody | UpdateHashTagBody | 否 |  |

## 响应

返回：[`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchHashTag200Response.ts)

## 示例

[inline-code-attrs-start title = 'patchHashTag 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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