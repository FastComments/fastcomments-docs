## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| largeInternalURLSanitized | string | Ναι |  |

## Response

Επιστρέφει: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifGetLargeResponse.ts)

## Example

[inline-code-attrs-start title = 'Παράδειγμα getGifLarge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
    const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";
    const largeInternalURLSanitized: string = "https://cdn.fastcomments.com/gifs/large/abc123.gif";
    const response: GifGetLargeResponse = await getGifLarge(tenantId, largeInternalURLSanitized);
    console.log(response);
}
[inline-code-end]