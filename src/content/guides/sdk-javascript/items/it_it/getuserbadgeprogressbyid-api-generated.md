## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeProgressResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getUserBadgeProgressById'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-tenant-72b1";
const badgeId: string = "badge-4d9f12";
const result: APIGetUserBadgeProgressResponse = await getUserBadgeProgressById(tenantId, badgeId);
const status: APIStatus | undefined = result?.status;
const progressList: UserBadgeProgress[] | undefined = result?.progress;
const firstProgress: UserBadgeProgress | undefined = progressList?.[0];
[inline-code-end]

---