Bulk-Benutzerinformationen für einen Mandanten. Angegebene userIds führen zu Anzeigeinformationen aus **User** / **SSOUser**.  
Wird vom Kommentar‑Widget verwendet, um Benutzer, die gerade über ein Präsenz‑Ereignis erschienen sind, anzureichern.  
Kein Seitenkontext: Datenschutz wird einheitlich durchgesetzt (private Profile werden maskiert).

## Parameter

| Name      | Typ    | Erforderlich | Beschreibung |
|-----------|--------|--------------|--------------|
| tenantId  | string | Ja           |  |
| ids       | string | Ja           |  |

## Antwort

Returns: [`GetUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfoResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getUsersInfo Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const ids: string = "user-1001,user-1002";

const usersInfo: GetUsersInfoResponse = await getUsersInfo(tenantId, ids);

// Optional fields in the response may be undefined
const firstUser: PageUserEntry | undefined = usersInfo?.users?.[0];
[inline-code-end]