एक इमेज अपलोड और रिसाइज़ करें

## पैरामीटर

| नाम | टाइप | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| file | Blob | हाँ |  |
| sizePreset | SizePreset | नहीं |  |
| urlId | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UploadImageResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'uploadImage उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-tenant-01";
const imageBlob: Blob = new Blob([new Uint8Array([137,80,78,71])], { type: "image/png" });

const uploadResult1: UploadImageResponse = await uploadImage(tenantId, imageBlob);

const sizePreset: SizePreset = { presetName: "medium" };
const urlId: string = "article-9876";

const uploadResult2: UploadImageResponse = await uploadImage(tenantId, imageBlob, sizePreset, urlId);
[inline-code-end]