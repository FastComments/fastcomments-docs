## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Sì |  |

## Risposta

Restituisce: [`UpdateUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserBadgeResponse.ts)

## Esempio

[inline-code-attrs-start title = 'updateUserBadge Esempio'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function applyBadge() {
    const tenantId: string = "acme-corp-tenant";
    const userId: string = "user-98765";

    const params: UpdateUserBadgeParams = {
        badgeId: "gold-contributor",
        // esempio di campo opzionale
        expiresAt: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString(),
    };

    const result: UpdateUserBadgeResponse = await updateUserBadge(tenantId, userId, params);
    console.log(result);
}

applyBadge();
[inline-code-end]