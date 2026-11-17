## 매개변수

| 이름 | 타입 | 필수 여부 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateQuestionResultBody | UpdateQuestionResultBody | 예 |  |

## 응답

반환값: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예제

[inline-code-attrs-start title = 'updateQuestionResult 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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