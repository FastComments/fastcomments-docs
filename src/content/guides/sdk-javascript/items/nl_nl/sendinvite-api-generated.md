## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| fromName | string | Ja |  |

## Antwoord

Retourneert: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'sendInvite Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'bright-media-12';
const id: string = 'user-8f4d2';
const fromName: string = 'Sofia Park';
const optionalNote: string | undefined = undefined;
const result: APIEmptyResponse = await sendInvite(tenantId, id, fromName);
[inline-code-end]

---