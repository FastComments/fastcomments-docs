उपयोग में आने वाले क्रेडेंशियल की पहचान करता है: जिस टेनेंट से वह संबंधित है और, OAuth टोकन के मामले में, वह उपयोगकर्ता जिसने इसे अधिकृत किया।
इंटीग्रेशन इसका उपयोग कनेक्शन का परीक्षण करने और उसे लेबल करने के लिए करते हैं।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetMeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMeResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getMe उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";
  const me: GetMeResponse = await getMe(tenantId);
  const authType: MeAuthType = me.auth.type;
  const scopes: OAuthScope[] = me.auth.scopes ?? [];
  const status: APIStatus = me.status;
})();
[inline-code-end]