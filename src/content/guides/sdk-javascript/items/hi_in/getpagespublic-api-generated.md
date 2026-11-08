किसी टेनेंट के लिए पेजों की सूची देता है। FChat डेस्कटॉप क्लाइंट द्वारा अपनी रूम सूची भरने के लिए उपयोग किया जाता है।
प्रत्येक पेज के लिए resolved custom config पर `enableFChat` true होना आवश्यक है।
जो पेज SSO की मांग करते हैं, उन्हें अनुरोध करने वाले उपयोगकर्ता के group access के आधार पर फ़िल्टर किया जाता है।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| cursor | string | नहीं |  |
| limit | number | नहीं |  |
| q | string | नहीं |  |
| sortBy | PagesSortBy | नहीं |  |
| hasComments | boolean | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getPagesPublic उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPublicPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "page_5";
  const limit: number = 20;
  const query: string = "support";
  const hasComments: boolean = true;

  const response: GetPublicPagesResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    query,
    undefined,
    hasComments
  );

  console.log(response);
}
[inline-code-end]