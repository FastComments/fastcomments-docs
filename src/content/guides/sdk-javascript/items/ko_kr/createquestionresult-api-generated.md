## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createQuestionResultBody | CreateQuestionResultBody | Yes |  |

## 응답

Returns: [`CreateQuestionResultResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResultResponse1.ts)

## 예시

[inline-code-attrs-start title = 'createQuestionResult 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";

const metaItem: MetaItem = {
  key: "campaign",
  value: "spring-launch"
};

const questionResultBody: CreateQuestionResultBody = {
  questionId: "question-42",
  answer: "Positive",
  metadata: [metaItem]
  // notes와 같은 선택적 필드는 생략되었습니다
};

const result: CreateQuestionResultResponse1 = await createQuestionResult(tenantId, questionResultBody);
[inline-code-end]

---