## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | number | 否 |  |

## 响应

返回: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigs200Response.ts)

## 示例

[inline-code-attrs-start title = 'getQuestionConfigs 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-42a7b9';
const firstPage: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId);
const secondPage: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId, 50);
[inline-code-end]

---