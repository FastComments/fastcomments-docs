## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createQuestionResultBody | CreateQuestionResultBody | Так |  |

## Відповідь

Повертає: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResultResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад createQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';

const createQuestionResultBody: CreateQuestionResultBody = {
  questionId: 'q-789',
  voterId: 'user-456',
  answerId: 'answer-yes',
  meta: { itemId: 'post-1234', itemType: 'blog-post' } as MetaItem
};

const response: CreateQuestionResultResponse = await createQuestionResult(
  tenantId,
  createQuestionResultBody
);

const status: APIStatus = response.status;
const questionResult: QuestionResult | undefined = response.questionResult;
console.log(`Created question result with status: ${status}`);
[inline-code-end]