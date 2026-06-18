## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfig200Response.ts)

## 예제

[inline-code-attrs-start title = 'getQuestionConfig 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-47";
const questionId: string = "q-4f2b9a";
const includeDrafts: boolean | undefined = undefined; // 선택적 매개변수 자리표시자
const result: GetQuestionConfig200Response = await getQuestionConfig(tenantId, questionId);
console.log(result);
[inline-code-end]

---