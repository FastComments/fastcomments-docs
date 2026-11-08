किसी टेनेंट के लिए पेजों की सूची लौटाता है। FChat डेस्कटॉप क्लाइंट द्वारा अपनी रूम सूची भरने के लिए उपयोग किया जाता है।
प्रत्येक पेज के लिए resolved custom config पर `enableFChat` true होना आवश्यक है।
SSO की आवश्यकता वाले पेजों को अनुरोध करने वाले उपयोगकर्ता की group access के आधार पर फ़िल्टर किया जाता है।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | GetPagesPublicOptions | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getPagesPublic उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]