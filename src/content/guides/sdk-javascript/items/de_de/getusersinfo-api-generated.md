Massen-Benutzerinformationen für einen tenant. Gibt für die angegebenen userIds Anzeigeinformationen aus User / SSOUser zurück.
Wird vom Kommentar-Widget verwendet, um Benutzer zu ergänzen, die gerade via a presence event erschienen sind.
Kein Seitenkontext: Die Privatsphäre wird einheitlich durchgesetzt (private Profile werden maskiert).

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Antwort

Gibt zurück: [`GetUsersInfo200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfo200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getUsersInfo Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-007';
const userIdsList: string[] = ['user_12a', 'user_34b', 'user_56c'];
const separator: string | undefined = undefined; // optional; wenn undefined, wird standardmäßig ein Komma verwendet
const ids: string = userIdsList.join(separator ?? ',');
const usersInfo: GetUsersInfo200Response = await getUsersInfo(tenantId, ids);
[inline-code-end]