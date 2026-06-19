## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Sì |  |

## Risposta

Restituisce: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptySuccessResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di updateUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-web-tenant-7";
const id: string = "badge_48f2a9";
const updateUserBadgeParams: UpdateUserBadgeParams = {
  label: "Community Champion",
  description: "Awarded for exceptional moderation and sustained helpful responses",
  active: true,
  expiresAt: "2026-12-31T23:59:59Z", // esempio di scadenza opzionale
  notifyUsers: true,
  metadata: { awardedBy: "moderator_jane" }
};
const result: APIEmptySuccessResponse = await updateUserBadge(tenantId, id, updateUserBadgeParams);
[inline-code-end]

---