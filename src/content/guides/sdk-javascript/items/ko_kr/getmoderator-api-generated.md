## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorResponse.ts)

## 예제

[inline-code-attrs-start title = 'getModerator 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-enterprises-72';
const id: string = 'mod_4b2f9a';
const response: GetModeratorResponse = await getModerator(tenantId, id);
const status: APIStatus | undefined = response.status;
const moderator: Moderator | undefined = response.moderator;
const moderatorEmail: string | undefined = response.moderator?.email;
[inline-code-end]

---