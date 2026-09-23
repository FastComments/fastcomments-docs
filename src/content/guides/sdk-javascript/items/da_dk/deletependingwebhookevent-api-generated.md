## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'deletePendingWebhookEvent Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeletion() {
  const tenantId: string = "tenant-42f7b9c2";
  const eventId: string = "webhook-event-8a7d6c5b";
  const result: APIEmptyResponse = await deletePendingWebhookEvent(tenantId, eventId);
  console.log(result);
}
[inline-code-end]