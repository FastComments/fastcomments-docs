## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| id | string | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## उदाहरण

[inline-code-attrs-start title = 'deleteV2PageReact उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeleteExamples() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "page_98765";
  const commentId: string = "comment_abcde";

  // वैकल्पिक sso के बिना कॉल करें
  const resultWithoutSso: CreateV1PageReact = await deleteV2PageReact(tenantId, urlId, commentId);

  // वैकल्पिक sso के साथ कॉल करें
  const ssoToken: string = "sso_token_xyz";
  const resultWithSso: CreateV1PageReact = await deleteV2PageReact(tenantId, urlId, commentId, ssoToken);
}
[inline-code-end]