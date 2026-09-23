---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| tag | string | 예 |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | 아니오 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예시

[inline-code-attrs-start title = 'deleteHashTag 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_42";
const tag: string = "high-priority";

const requestBody: DeleteHashTagRequestBody = {
  // 필요한 필드를 채워 주세요
};

const resultWithBody: APIEmptyResponse = await deleteHashTag(tenantId, tag, requestBody);
const resultWithoutBody: APIEmptyResponse = await deleteHashTag(tenantId, tag);
[inline-code-end]

---