## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentIds | string | Ja |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CheckBlockedCommentsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'checkedCommentsForBlocked Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const commentIds: string = 'cmt_1001,cmt_1002';
const resultWithoutSso: CheckBlockedCommentsResponse = await checkedCommentsForBlocked(tenantId, commentIds);

const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.dummy.payload';
const resultWithSso: CheckBlockedCommentsResponse = await checkedCommentsForBlocked(tenantId, commentIds, sso);
[inline-code-end]

---