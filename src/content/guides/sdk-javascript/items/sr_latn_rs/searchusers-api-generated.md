## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| usernameStartsWith | string | Ne |  |
| mentionGroupIds | Array<string> | Ne |  |
| sso | string | Ne |  |
| searchSection | SearchUsersSearchSectionEnum | Ne |  |

## Odgovor

Vraća: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SearchUsers200Response.ts)

## Primer

[inline-code-attrs-start title = 'searchUsers Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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