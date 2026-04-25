## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| createQuestionResultBody | CreateQuestionResultBody | はい |  |

## レスポンス

戻り値: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResult200Response.ts)

## 例

[inline-code-attrs-start title = 'createQuestionResult の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-tenant-01';
const createQuestionResultBody: CreateQuestionResultBody = {
  questionId: 'q-34567',
  respondentId: 'user-8923',
  answers: [{ optionId: 'opt_A', text: 'Agree', count: 1 }],
  score: 5,
  meta: [{ key: 'platform', value: 'web' }],
  notifyModerators: false // 任意のパラメータ
} as CreateQuestionResultBody;
const result: CreateQuestionResult200Response = await createQuestionResult(tenantId, createQuestionResultBody);
[inline-code-end]

---