## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| tag | string | हाँ |  |
| updateHashTagBody | UpdateHashTagBody | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'patchHashTag उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const tag: string = "news";

  // वैकल्पिक बॉडी के बिना कॉल करें
  const responseWithoutBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag);

  // अपडेट के लिए बॉडी तैयार करें
  const updateBody: UpdateHashTagBody = {
    name: "Latest News",
    description: "Tag for the most recent news articles"
  };

  const responseWithBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag, updateBody);
})();
[inline-code-end]