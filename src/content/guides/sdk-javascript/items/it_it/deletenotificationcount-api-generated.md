## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f3d2e1b';
const id: string = 'notification_9a4b1c2';
const result: FlagCommentPublic200Response = await deleteNotificationCount(tenantId, id);
[inline-code-end]

---