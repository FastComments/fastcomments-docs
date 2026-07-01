## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| urlIdWS | string | Ja |  |
| userIds | string | Ja |  |

## Respons

Returns: [`GetUserPresenceStatusesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserPresenceStatusesResponse1.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getUserPresenceStatuses Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUserPresences() {
  const tenantId: string = "tenant_9f8e2d";
  const urlIdWS: string = "blog.mycompany.com/thread/12345";
  const userIds: string = "alice,bob,carol";

  const result: GetUserPresenceStatusesResponse1 = await getUserPresenceStatuses(
    tenantId,
    urlIdWS,
    userIds
  );

  console.log(result);
}
[inline-code-end]

---