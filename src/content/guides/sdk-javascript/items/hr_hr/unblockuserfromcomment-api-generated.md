## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Da |  |
| userId | string | Ne |  |
| anonUserId | string | Ne |  |

## Odgovor

Vraća: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnblockSuccess.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer unBlockUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8b4a2f9c';
const id: string = 'cmt_5f3b2a9e';
const unBlockFromCommentParams: UnBlockFromCommentParams = { reason: 'Appeal accepted', effectiveAt: '2026-06-19T12:00:00Z' };
const userId: string = 'user_42f7';
const result: UnblockSuccess = await unBlockUserFromComment(tenantId, id, unBlockFromCommentParams, userId);
[inline-code-end]

---