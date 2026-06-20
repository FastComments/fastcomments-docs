---
## Parameter

| Name | Type | Erforderlich | Beschreibung |
|------|------|--------------|-------------|
| commentId | string | Ja |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`ModerationAPIGetLogsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetLogsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getLogs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const commentId: string = 'cmt_0f9b1a2c3d4e5f6a';
  const logsWithoutSSO: ModerationAPIGetLogsResponse = await getLogs(commentId);
  const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1Njc4OSJ9.dQw4w9WgXcQ';
  const logsWithSSO: ModerationAPIGetLogsResponse = await getLogs(commentId, ssoToken);
  console.log(logsWithoutSSO, logsWithSSO);
})();
[inline-code-end]

---