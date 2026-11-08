## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| updateAPIPageData | UpdateAPIPageData | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchPageAPIResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'patchPage उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const pageId: string = "page_98765";

  const updateData: UpdateAPIPageData = {
    title: "Updated FAQ Page"
    // description?: string को छोड़ा जा सकता है
  };

  const response: PatchPageAPIResponse = await patchPage(tenantId, pageId, updateData);
})();
[inline-code-end]