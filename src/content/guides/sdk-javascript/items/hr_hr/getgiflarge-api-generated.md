## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| largeInternalURLSanitized | string | Da |  |

## Odgovor

Vraća: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifGetLargeResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getGifLarge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-42';
const largeInternalURLSanitized: string = '/assets/internal/gifs/launch-party-9f8b7c.gif';
const correlationId?: string = 'req-20260619-01';
const result: GifGetLargeResponse = await getGifLarge(tenantId, largeInternalURLSanitized);
[inline-code-end]

---