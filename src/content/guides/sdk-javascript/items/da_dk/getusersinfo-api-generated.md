Bulk brugerinformation for en tenant. Givet userIds, returner visningsoplysninger fra User / SSOUser.
Bruges af kommentar-widgeten til at berige brugere, der lige er dukket op via en presence-begivenhed.
Ingen sidekontekst: privatliv håndhæves ensartet (private profiler maskeres).

## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| ids | string | Ja |  |

## Respons

Returnerer: [`GetUsersInfo200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfo200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'getUsersInfo Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-007';
const userIdsList: string[] = ['user_12a', 'user_34b', 'user_56c'];
const separator: string | undefined = undefined; // optional; if undefined default to comma
const ids: string = userIdsList.join(separator ?? ',');
const usersInfo: GetUsersInfo200Response = await getUsersInfo(tenantId, ids);
[inline-code-end]