## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPIPageData | CreateAPIPageData | Yes |  |

## Response

Returns: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddPageAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'addPage Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_8f3b2a9c-prod";
  const createAPIPageData: CreateAPIPageData = {
    url: "https://acme.blog/products/fastsync-2",
    title: "Introducing FastSync 2.0",
    slug: "fastsync-2-0",
    tags: ["releases", "sync", "performance"],
    isPublic: true,
    language: "en-US",
    canonicalUrl: "https://acme.com/products/fastsync-2", // optional
    metadata: { authorId: "user_42", summary: "Faster sync across devices." } // optional
  };
  const result: AddPageAPIResponse = await addPage(tenantId, createAPIPageData);
  console.log((result as any).page as APIPage);
})();
[inline-code-end]
