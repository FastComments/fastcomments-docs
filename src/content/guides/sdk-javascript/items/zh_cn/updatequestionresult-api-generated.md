## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateQuestionResultBody | UpdateQuestionResultBody | 是 |  |

## 响应

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 示例

[inline-code-attrs-start title = 'updateQuestionResult 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme-corp_01";
const id: string = "question_9f2d1b";
const metaItem: MetaItem = { key: "platform", value: "web" };
const status: APIStatus = { code: 0, message: "scored" };
const updateQuestionResultBody: UpdateQuestionResultBody = {
  score: 92,
  passed: true,
  status,
  meta: [metaItem] // 可选字段示例
};
const result: APIEmptyResponse = await updateQuestionResult(tenantId, id, updateQuestionResultBody);
[inline-code-end]

---