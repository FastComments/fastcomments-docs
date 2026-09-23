## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| urlIdWS | string | Ja |  |
| userIds | string | Ja |  |

## Antwort

Rückgabe: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserPresenceStatusesResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getUserPresenceStatuses Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "my-tenant-42";
  const urlIdWS: string = "post-9876";
  const userIds: string = "alice,bob,carol";

  const presence: GetUserPresenceStatusesResponse = await getUserPresenceStatuses(tenantId, urlIdWS, userIds);
  console.log(presence);
})();
[inline-code-end]

---