Sammel-Benutzerinformationen für einen Mandanten. Gegeben die userIds, werden Anzeigeinformationen aus User / SSOUser zurückgegeben.
Wird vom Kommentar-Widget verwendet, um Benutzer zu ergänzen, die gerade durch ein Presence-Ereignis erschienen sind.
Kein Seitenkontext: Datenschutz wird einheitlich durchgesetzt (private Profile werden maskiert).

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| ids | string | Ja |  |

## Antwort

Gibt zurück: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getUsersInfo Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo benötigt nur tenantId und ids; optionale Parameter sind hier nicht anwendbar.
[inline-code-end]

---