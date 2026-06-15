## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Svar

Returnerer: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserBadge200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på deleteUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8b3f2c7a";
const badgeIdOptional: string | undefined = Math.random() > 0.5 ? "badge_4f9a21" : undefined;
const id: string = badgeIdOptional ?? "badge_backup_01";
const result: UpdateUserBadge200Response = await deleteUserBadge(tenantId, id);
[inline-code-end]

---