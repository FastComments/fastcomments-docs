## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | いいえ |  |
| userId | string | いいえ |  |
| startDate | string | いいえ |  |
| questionId | string | いいえ |  |
| questionIds | string | いいえ |  |
| skip | number | いいえ |  |

## レスポンス

戻り値: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultsResponse.ts)

## 例

[inline-code-attrs-start title = 'getQuestionResults の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare const getQuestionResults: (
  tenantId: string,
  urlId?: string,
  userId?: string,
  startDate?: string,
  questionId?: string,
  questionIds?: string,
  skip?: number
) => Promise<GetQuestionResultsResponse>;

const tenantId: string = 'abc123def456';
const urlId: string = 'https://example.com/blog/my-post';
const userId: string = 'user_98765';
const startDate: string = '2024-01-01T00:00:00.000Z';
const skip: number = 0;

const results: GetQuestionResultsResponse = await getQuestionResults(
  tenantId,
  urlId,
  userId,
  startDate,
  undefined,
  'question_1,question_2',
  skip
);

const firstResult: QuestionResult | undefined = results[0];
[inline-code-end]