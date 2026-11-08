## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| includeByUserIdAndEmail | boolean | नहीं |  |
| includeByIP | boolean | नहीं |  |
| includeByEmailDomain | boolean | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`PreBanSummary`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PreBanSummary.ts)

## उदाहरण

[inline-code-attrs-start title = 'getPreBanSummary उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchSummary(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const includeByUserIdAndEmail: boolean = true;
  const includeByIP: boolean = false;
  const includeByEmailDomain: boolean = true;
  const sso: string = "sso_token_abc";

  const summary: PreBanSummary = await getPreBanSummary(
    tenantId,
    commentId,
    includeByUserIdAndEmail,
    includeByIP,
    includeByEmailDomain,
    sso
  );

  console.log(summary);
}

fetchSummary();
[inline-code-end]