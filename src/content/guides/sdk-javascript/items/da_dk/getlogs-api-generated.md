## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| sso | string | No |  |

## Svar

Returnerer: [`ModerationAPIGetLogsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetLogsResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getLogs Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-123";
  const commentId: string = "cmt_4567890";
  const ssoToken: string = "sso_abcdef123456";

  const logsWithSso: ModerationAPIGetLogsResponse = await getLogs(tenantId, commentId, ssoToken);
  const logsWithoutSso: ModerationAPIGetLogsResponse = await getLogs(tenantId, commentId);
})();
[inline-code-end]