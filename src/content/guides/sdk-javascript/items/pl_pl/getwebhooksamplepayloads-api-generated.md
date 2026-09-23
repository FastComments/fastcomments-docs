Ostatnie komentarze w dokładnym formacie używanym przez dostawy webhooków, służące do budowania integracji (na przykład przykładowe dane Zapier). Każde zdarzenie dostarcza ten sam obiekt komentarza, więc `event` musi być jedynie prawidłowy.

## Parameters

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| event | WebhookEventName | No |  |
| limit | number | No |  |

## Odpowiedź

Zwraca: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getWebhookSamplePayloads'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";

const responseAll: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  "comment.created",
  10
);

const responseWithEvent: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  "comment.created"
);

const responseBasic: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId
);
[inline-code-end]