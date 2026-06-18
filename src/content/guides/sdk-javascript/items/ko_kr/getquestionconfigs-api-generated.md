## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| skip | number | 아니오 |  |

## 응답

반환: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigs200Response.ts)

## 예제

[inline-code-attrs-start title = 'getQuestionConfigs 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4c9f2b";
const responseWithoutSkip: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId);
const skip: number = 50;
const responseWithSkip: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId, skip);
[inline-code-end]

---