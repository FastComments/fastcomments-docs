---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUser200Response.ts)

## 예제

[inline-code-attrs-start title = 'getUser 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const idSuffix: string | undefined = undefined;
const tenantId: string = "acme-enterprises";
const id: string = idSuffix ?? "user_98765";
const response: GetUser200Response = await getUser({ tenantId, id });
[inline-code-end]

---