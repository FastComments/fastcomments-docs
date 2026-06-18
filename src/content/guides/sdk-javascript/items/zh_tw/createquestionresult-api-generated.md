## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createQuestionResultBody | CreateQuestionResultBody | 是 |  |

## 回應

回傳: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResult200Response.ts)

## 範例

[inline-code-attrs-start title = 'createQuestionResult 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fc-tenant-512';
const meta: MetaItem[] = [{ key: 'source', value: 'article' }];
const body: CreateQuestionResultBody = {
  questionId: 'q-94',
  userId: 'user_332',
  answers: [{ optionId: 'opt_a', score: 1 }],
  meta, // 提供的選用元資料
} as CreateQuestionResultBody;
const result: CreateQuestionResult200Response = await createQuestionResult(tenantId, body);
[inline-code-end]