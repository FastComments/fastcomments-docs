## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfig200Response.ts)

## 예제

[inline-code-attrs-start title = 'getQuestionConfig 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-47';
const questionId: string = 'q-2026-01-12-01';
const result: GetQuestionConfig200Response = await getQuestionConfig(tenantId, questionId);
function summarizeConfig(cfg: GetQuestionConfig200Response, includeMetadata?: boolean): QuestionConfig | undefined {
  // includeMetadata는 선택 사항입니다; 간결성을 위해 구현은 생략되었습니다
  return undefined;
}
const summarizedWithMeta: QuestionConfig | undefined = summarizeConfig(result, true);
const summarizedDefault: QuestionConfig | undefined = summarizeConfig(result);
[inline-code-end]

---