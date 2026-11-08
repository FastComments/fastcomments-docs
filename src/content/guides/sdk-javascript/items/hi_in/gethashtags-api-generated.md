## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| page | number | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTagsResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getHashTags उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-corp-123";
    const page: number = 1;
    const result: GetHashTagsResponse = await getHashTags(tenantId, page);
    console.log(result);
})();
[inline-code-end]