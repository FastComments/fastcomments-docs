Verwijder een poll uit zijn reactie, samen met elke stem die erop is uitgebracht. De reactie zelf blijft onaangetast.

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|---------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |

## Respons

Retourneert: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'deletePoll Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeletePoll(): Promise<void> {
  const tenantId: string = "tenant_9f8b7c6d";
  const commentId: string = "comment_a1b2c3d4";
  const result: APIEmptyResponse = await deletePoll(tenantId, commentId);
  console.log(result);
}
[inline-code-end]