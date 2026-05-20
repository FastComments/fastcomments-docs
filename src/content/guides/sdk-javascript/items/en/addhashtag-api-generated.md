## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | No |  |
| createHashTagBody | CreateHashTagBody | No |  |

## Response

Returns: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTag200Response.ts)

## Example

[inline-code-attrs-start title = 'addHashTag Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_4821";
  const createHashTagBody: CreateHashTagBody = { name: "product-feedback", title: "Product Feedback", colorHex: "#1f8ef1", isActive: true };
  const resultFull: AddHashTag200Response = await addHashTag(tenantId, createHashTagBody);
  const resultTenantOnly: AddHashTag200Response = await addHashTag(tenantId);
  console.log(resultFull, resultTenantOnly);
})();
[inline-code-end]
