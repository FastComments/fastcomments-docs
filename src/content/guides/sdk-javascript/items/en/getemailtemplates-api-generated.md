## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Response

Returns: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplates200Response.ts)

## Example

[inline-code-attrs-start title = 'getEmailTemplates Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-4f7c2b9a";
  const templatesDefault: GetEmailTemplates200Response = await getEmailTemplates(tenantId);
  const templatesWithSkip: GetEmailTemplates200Response = await getEmailTemplates(tenantId, 25);
  console.log(templatesDefault, templatesWithSkip);
})();
[inline-code-end]
