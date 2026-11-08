## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| largeInternalURLSanitized | string | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifGetLargeResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getGifLarge उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
    const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";
    const largeInternalURLSanitized: string = "https://cdn.fastcomments.com/gifs/large/abc123.gif";
    const response: GifGetLargeResponse = await getGifLarge(tenantId, largeInternalURLSanitized);
    console.log(response);
}
[inline-code-end]