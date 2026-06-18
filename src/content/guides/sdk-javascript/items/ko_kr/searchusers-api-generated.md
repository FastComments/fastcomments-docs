## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| usernameStartsWith | string | 아니오 |  |
| mentionGroupIds | Array<string> | 아니오 |  |
| sso | string | 아니오 |  |
| searchSection | SearchUsersSearchSectionEnum | 아니오 |  |

## 응답

반환: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SearchUsers200Response.ts)

## 예제

[inline-code-attrs-start title = 'searchUsers 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7890';
const urlId: string = 'news/2026-06-15-world-cup-final';
const usernameStartsWith: string = 'mar';
const mentionGroupIds: string[] = ['staff', 'trusted-commenters'];
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI0NTY3OCJ9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
const searchSection: SearchUsersSearchSectionEnum = SearchUsersSearchSectionEnum.Commenters;
const response: SearchUsers200Response = await searchUsers(tenantId, urlId, usernameStartsWith, mentionGroupIds, sso, searchSection);
[inline-code-end]

---