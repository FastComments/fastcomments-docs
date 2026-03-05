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
const tenantId: string = "acme-corp-prod-01";
const createData: CreateAPIPageData = {
  url: "/docs/2026/comment-moderation",
  title: "Comment Moderation Guide",
  description: "Step-by-step instructions to configure moderation workflows for community posts.",
  language: "en-US", // optional field
  metadata: { productArea: "support", owner: "platform-team" } // optional field
};
const result: AddPageAPIResponse = await addPage(tenantId, createData);
[inline-code-end]
