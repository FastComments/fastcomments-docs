## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| largeInternalURLSanitized | string | Да |  |

## Отговор

Връща: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifGetLargeResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getGifLarge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8a92f4';
const largeInternalURLSanitized: string = 'https://cdn.streamingco.com/gifs/product-demo-large.gif';
let maybeStatus: APIStatus | undefined = undefined; // незадължителни метаданни, когато са налични
const response: GifGetLargeResponse = await getGifLarge(tenantId, largeInternalURLSanitized);
[inline-code-end]

---