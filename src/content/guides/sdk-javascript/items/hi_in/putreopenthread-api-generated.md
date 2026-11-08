## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'putReopenThread उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function reopenThreadExample() {
    const tenantId: string = "tenant_12345";
    const urlId: string = "post-9876";
    const sso: string = "user-abc123";

    const responseWithoutSso: APIEmptyResponse = await putReopenThread(tenantId, urlId);
    const responseWithSso: APIEmptyResponse = await putReopenThread(tenantId, urlId, sso);
}

reopenThreadExample();
[inline-code-end]