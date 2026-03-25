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
const tenantId: string = 'tenant_8392';
const urlId: string = 'articles/2026/03/25/fastcomments-release';
const usernameStartsWith: string = 'jo';
const mentionGroupIds: Array<string> = ['editors', 'senior-writers'];
const sso: string = 'sso_jwt_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const searchSection: SearchUsersSearchSectionEnum = SearchUsersSearchSectionEnum.ALL;
const result: SearchUsers200Response = await searchUsers(tenantId, urlId, usernameStartsWith, mentionGroupIds, sso, searchSection);
[inline-code-end]

---