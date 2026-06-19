---
## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Yes |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예제

[inline-code-attrs-start title = 'updateQuestionResult 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme-corp_01";
const id: string = "question_9f2d1b";
const metaItem: MetaItem = { key: "platform", value: "web" };
const status: APIStatus = { code: 0, message: "scored" };
const updateQuestionResultBody: UpdateQuestionResultBody = {
  score: 92,
  passed: true,
  status,
  meta: [metaItem] // 선택적 필드 예시
};
const result: APIEmptyResponse = await updateQuestionResult(tenantId, id, updateQuestionResultBody);
[inline-code-end]

---