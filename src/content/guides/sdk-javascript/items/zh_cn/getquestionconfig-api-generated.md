## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfig200Response.ts)

## 示例

[inline-code-attrs-start title = 'getQuestionConfig 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-47";
const questionId: string = "q-4f2b9a";
const includeDrafts: boolean | undefined = undefined; // 可选参数占位符
const result: GetQuestionConfig200Response = await getQuestionConfig(tenantId, questionId);
console.log(result);
[inline-code-end]

---