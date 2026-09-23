Entfernen Sie eine Umfrage aus ihrem Kommentar, zusammen mit allen abgegebenen Stimmen. Der Kommentar selbst bleibt unverändert.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|-----|--------------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |

## Antwort

Rückgabe: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'deletePoll Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeletePoll(): Promise<void> {
  const tenantId: string = "tenant_9f8b7c6d";
  const commentId: string = "comment_a1b2c3d4";
  const result: APIEmptyResponse = await deletePoll(tenantId, commentId);
  console.log(result);
}
[inline-code-end]

---