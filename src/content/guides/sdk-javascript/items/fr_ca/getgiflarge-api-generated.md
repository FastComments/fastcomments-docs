## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| largeInternalURLSanitized | string | Oui |  |

## Réponse

Retourne: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifGetLargeResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getGifLarge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-42';
const largeInternalURLSanitized: string = '/assets/internal/gifs/launch-party-9f8b7c.gif';
const correlationId?: string = 'req-20260619-01';
const result: GifGetLargeResponse = await getGifLarge(tenantId, largeInternalURLSanitized);
[inline-code-end]

---