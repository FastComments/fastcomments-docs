## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Svar

Returnerer: [`GetUserNotificationCountResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationCountResponse1.ts)

## Eksempel

[inline-code-attrs-start title = 'getUserNotificationCount Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetUserNotificationCount() {
    const tenantId: string = "acme-corp-01";

    // Kald med valgfri SSO-token
    const countWithSSO: GetUserNotificationCountResponse1 = await getUserNotificationCount(
        tenantId,
        "sso-token-abc123"
    );

    // Kald uden SSO-token
    const countWithoutSSO: GetUserNotificationCountResponse1 = await getUserNotificationCount(
        tenantId
    );

    console.log(countWithSSO, countWithoutSSO);
}
[inline-code-end]