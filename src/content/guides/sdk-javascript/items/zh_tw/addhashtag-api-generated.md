## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 否 |  |
| createHashTagBody | CreateHashTagBody | 否 |  |

## 回應

回傳：[`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateHashTagResponse.ts)

## 範例

[inline-code-attrs-start title = 'addHashTag 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_8a4f2c";
  const createHashTagBody: CreateHashTagBody = {
    name: "performance-issue",
    displayName: "Performance Issue",
    color: "#FF4500",
    isActive: true,
    priority: 5
  };
  const response: CreateHashTagResponse = await addHashTag(tenantId, createHashTagBody);
  const responseDefaultTenant: CreateHashTagResponse = await addHashTag(undefined, createHashTagBody);
  console.log(response, responseDefaultTenant);
})();
[inline-code-end]

---