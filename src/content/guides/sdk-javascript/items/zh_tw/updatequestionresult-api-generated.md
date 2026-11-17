## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateQuestionResultBody | UpdateQuestionResultBody | 是 |  |

## 回應

回傳:[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 範例

[inline-code-attrs-start title = 'updateQuestionResult 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a2b6c7f8d9e0a1b2c3d4e5f';
const questionId: string = 'q-8f3b9a2c-41d7-4e6a-b5c8-9e0f1a2b3c4d';

const updateBody: UpdateQuestionResultBody = {
  correctAnswer: 'Option B: Blue whale',
  isAnsweredCorrectly: true,
  answeredAt: '2024-03-15T14:32:00.000Z',
  meta: [
    {
      key: 'quiz_session',
      value: 'session_abc123'
    }
  ]
};

const result: APIEmptyResponse = await updateQuestionResult(tenantId, questionId, updateBody);

console.log(`Update status: ${result.status}`);
[inline-code-end]