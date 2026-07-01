Past commenters on the page who are NOT currently online. Sorted by displayName.  
Verwenden Sie dies, nachdem /users/online vollständig abgefragt wurde, um einen "Members"-Abschnitt darzustellen.  
Cursor-Paginierung nach commenterName: Der Server durchläuft das Teilobjekt {tenantId, urlId, commenterName}  
Index ab afterName vorwärts mittels $gt, ohne $skip Kosten.

## Parameters

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nein |  |
| afterUserId | string | Nein |  |

## Response

Returns: [`GetOfflineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsersResponse.ts)

## Example

[inline-code-attrs-start title = 'getOfflineUsers Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOfflineUsers(): Promise<void> {
    const tenantId: string = "tenant_12345";
    const urlId: string = "thread_9876";
    const afterName: string = "Jane Smith";
    const afterUserId: string = "user_7f9b3c";

    const offlineUsers: GetOfflineUsersResponse = await getOfflineUsers(
        tenantId,
        urlId,
        afterName,
        afterUserId
    );

    console.log(offlineUsers);
}
[inline-code-end]