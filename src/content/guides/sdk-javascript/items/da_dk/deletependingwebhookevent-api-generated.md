## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'deletePendingWebhookEvent Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removePendingWebhookEvent(tenantId?: string): Promise<APIEmptyResponse | undefined> {
  if (!tenantId) return;
  const tenant: string = tenantId;
  const eventId: string = 'evt_7f2c1a9b';
  const response: APIEmptyResponse = await deletePendingWebhookEvent(tenant, eventId);
  return response;
}
[inline-code-end]

---