## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| banUserUndoParams | BanUserUndoParams | Ja |  |
| sso | string | Nej |  |

## Svar

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'postBanUserUndo Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9876";

const undoParams: BanUserUndoParams = {
  userId: "user_12345",
  // yderligere påkrævede felter kan tilføjes her
};

const resultWithoutSSO: APIEmptyResponse = await postBanUserUndo(tenantId, undoParams);

const ssoToken: string = "sso_abcdef123456";

const resultWithSSO: APIEmptyResponse = await postBanUserUndo(tenantId, undoParams, ssoToken);
[inline-code-end]