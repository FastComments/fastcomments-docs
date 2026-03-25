## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createQuestionResultBody | CreateQuestionResultBody | 是 |  |

## 响应

返回: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResult200Response.ts)

## 示例

[inline-code-attrs-start title = 'createQuestionResult 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-72b1f4";
const meta: MetaItem[] = [{ key: "platform", value: "web" }];
const createQuestionResultBody: CreateQuestionResultBody = {
  questionId: "question-83472",
  commenterId: "user-5521",
  answers: [{ subQuestionId: "sq-1", value: "Yes" }],
  meta, // 可选的元数据
  note: "Follow-up requested" // 演示可选参数
};
const result: CreateQuestionResult200Response = await createQuestionResult(tenantId, createQuestionResultBody);
[inline-code-end]

---