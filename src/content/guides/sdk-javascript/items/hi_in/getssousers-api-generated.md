## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| skip | number | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUsersResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getSSOUsers उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample(): Promise<void> {
    const tenantId: string = "tenant_12345";

    // वैकल्पिक `skip` के बिना कॉल करें
    const firstPage: GetSSOUsersResponse = await getSSOUsers(tenantId);

    // वैकल्पिक `skip` के साथ कॉल करें
    const secondPage: GetSSOUsersResponse = await getSSOUsers(tenantId, 100);

    console.log(firstPage, secondPage);
}

runExample();
[inline-code-end]