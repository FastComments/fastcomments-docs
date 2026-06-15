---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| blockFromCommentParams | BlockFromCommentParams | Ja |  |
| userId | string | Nein |  |
| anonUserId | string | Nein |  |

## Antwort

Gibt zurück: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublic200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'blockUserFromComment Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f3b4c";
const id: string = "comment_9a8b7c6d";
const blockFromCommentParams: BlockFromCommentParams = {
  reason: "Repeated spam links",
  durationHours: 168,
  notifyModerators: true
};
const userId: string | undefined = "user_42";
const anonUserId: string | undefined = undefined;
const result: BlockFromCommentPublic200Response = await blockUserFromComment(tenantId, id, blockFromCommentParams, userId, anonUserId);
[inline-code-end]

---