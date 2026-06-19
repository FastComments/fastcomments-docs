## Parameter

| Name | Type | Erforderlich | Beschreibung |
|------|------|--------------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| userId | string | Nein |  |
| anonUserId | string | Nein |  |

## Antwort

Gibt zurück: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'unFlagComment Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-001';
const id: string = 'cmt-9b8f7d6a5';
const userId: string = 'user-42a7c9e1';

const result: FlagCommentResponse = await unFlagComment(tenantId, id, userId);
[inline-code-end]

---