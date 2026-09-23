## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| tag | string | Yes |  |
| updateHashTagBody | UpdateHashTagBody | No |  |

## 回應

返回：[`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## 範例

[inline-code-attrs-start title = 'patchHashTag 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const tag: string = "news";

  // 呼叫時未提供可選的 body
  const responseWithoutBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag);

  // 準備更新的 body
  const updateBody: UpdateHashTagBody = {
    name: "Latest News",
    description: "Tag for the most recent news articles"
  };

  const responseWithBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag, updateBody);
})();
[inline-code-end]

---