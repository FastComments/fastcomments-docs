Frühere Kommentierende auf der Seite, die derzeit NICHT online sind. Nach displayName sortiert.
Verwenden Sie dies, nachdem Sie /users/online ausgeschöpft haben, um einen "Mitglieder"-Abschnitt darzustellen.
Cursor-Paginierung auf commenterName: Der Server durchläuft den partiellen {tenantId, urlId, commenterName}-Index von afterName aus vorwärts über $gt, keine $skip-Kosten.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nein |  |
| afterUserId | string | Nein |  |

## Antwort

Gibt zurück: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9f4b2a6c';
const urlId: string = 'articles/product-launch-2025';

const offlinePageFirst: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId);

const afterName: string = 'samantha.r';
const afterUserId: string = 'user_7d3a21f9';
const offlinePageNext: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---