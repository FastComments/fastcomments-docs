## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| usernameStartsWith | string | Όχι |  |
| mentionGroupIds | Array<string> | Όχι |  |
| sso | string | Όχι |  |
| searchSection | SearchUsersSearchSectionEnum | Όχι |  |

## Απόκριση

Επιστρέφει: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SearchUsers200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα searchUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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