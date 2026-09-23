## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| fromName | string | Yes |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예시

[inline-code-attrs-start title = 'sendInvite 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-987654321";
const id: string = "user-123456789";
const fromName: string = "Alice Johnson";

const result: APIEmptyResponse = await sendInvite(tenantId, id, fromName);
console.log(result);
[inline-code-end]