एक छवि अपलोड और रिसाइज़ करें

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| file | string | नहीं |  |
| options | UploadImageOptions | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`Option[UploadImageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_upload_image_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'uploadImage उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (uploadResult, httpResponse) = client.uploadImage(
  tenantId = "my-tenant-123",
  file = "images/avatar.jpg",
  options = UploadImageOptions()
)

if uploadResult.isSome:
  let result = uploadResult.get()
  # result का आवश्यकतानुसार उपयोग करें
[inline-code-end]