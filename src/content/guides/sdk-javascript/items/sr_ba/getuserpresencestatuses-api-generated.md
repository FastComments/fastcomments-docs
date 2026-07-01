## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlIdWS | string | Yes |  |
| userIds | string | Yes |  |

## Odgovor

Vraća: [`GetUserPresenceStatusesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserPresenceStatusesResponse1.ts)

## Primjer

[inline-code-attrs-start title = 'getUserPresenceStatuses Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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