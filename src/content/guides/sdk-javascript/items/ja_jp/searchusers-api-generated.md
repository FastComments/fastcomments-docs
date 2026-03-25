## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| usernameStartsWith | string | いいえ |  |
| mentionGroupIds | Array<string> | いいえ |  |
| sso | string | いいえ |  |
| searchSection | SearchUsersSearchSectionEnum | いいえ |  |

## レスポンス

戻り値: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SearchUsers200Response.ts)

## 例

[inline-code-attrs-start title = 'searchUsers の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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