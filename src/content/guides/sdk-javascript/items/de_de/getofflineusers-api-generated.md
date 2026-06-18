Frühere Kommentatoren auf der Seite, die DERZEIT NICHT online sind. Nach displayName sortiert.
Verwenden Sie dies nachdem Sie /users/online ausgeschöpft haben, um einen "Members"-Abschnitt darzustellen.
Cursor-Pagination auf commenterName: der Server durchläuft den partiellen {tenantId, urlId, commenterName}
Index ab afterName vorwärts mittels $gt, keine $skip-Kosten.

## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nein |  |
| afterUserId | string | Nein |  |

## Antwort

Gibt zurück: [`GetOfflineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsers200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getOfflineUsers Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_001';
const urlId: string = 'article-2026-06-15-how-ai-impacts';
const afterName: string = 'michael.smith';
const afterUserId: string = 'user_72b9';

const response: GetOfflineUsers200Response = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]