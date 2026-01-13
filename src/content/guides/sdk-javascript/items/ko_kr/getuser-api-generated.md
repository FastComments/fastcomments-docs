## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUser200Response.ts)

## 예제

[inline-code-attrs-start title = 'getUser 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-publishing-42";
const userIdOptional: string | undefined = "user_9d7b4c"; // 일부 흐름에서는 undefined일 수 있음(선택 사항)
const id: string = userIdOptional ?? "user_default_0001";
const result: GetUser200Response = await getUser(tenantId, id);
console.log(result);
[inline-code-end]

---