## 参数

| Name | Type | Required | 描述 |
|------|------|----------|------|
| tenantId | string | 是 |  |
| skip | number | 否 |  |

## 响应

返回：[`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigsResponse.ts)

## 示例

[inline-code-attrs-start title = 'getQuestionConfigs 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchConfigs() {
  const tenantId: string = "tenant_12345";

  const firstPage: GetQuestionConfigsResponse = await getQuestionConfigs(tenantId);
  const secondPage: GetQuestionConfigsResponse = await getQuestionConfigs(tenantId, 20);
}
[inline-code-end]

---