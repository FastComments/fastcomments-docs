## 參數

| 名稱 | 類型 | 必要 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createQuestionResultBody | CreateQuestionResultBody | Yes |  |

## 回應

回傳: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResult200Response.ts)

## 範例

[inline-code-attrs-start title = 'createQuestionResult 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-tenant-01';
const createQuestionResultBody: CreateQuestionResultBody = {
  questionId: 'q-34567',
  respondentId: 'user-8923',
  answers: [{ optionId: 'opt_A', text: 'Agree', count: 1 }],
  score: 5,
  meta: [{ key: 'platform', value: 'web' }],
  notifyModerators: false // 可選參數
} as CreateQuestionResultBody;
const result: CreateQuestionResult200Response = await createQuestionResult(tenantId, createQuestionResultBody);
[inline-code-end]

---