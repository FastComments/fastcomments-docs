## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | Yes |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Yes |  |

## Response

Returns: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfig200Response.ts)

## Example

[inline-code-attrs-start title = 'putDomainConfig Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const domainToUpdate: string = 'comments.acme.com';
const updateDomainConfigParams: UpdateDomainConfigParams = {
  enableSsl: true,
  certificateProvider: 'letsencrypt',
  redirectHttpToHttps: true, // optional parameter demonstrated
  customCssUrl: 'https://cdn.acme.com/widgets/comments.css',
  moderationMode: 'pre-moderation',
  allowedOrigins: ['https://www.acme.com', 'https://blog.acme.com']
};
const updatedConfig: GetDomainConfig200Response = await putDomainConfig(tenantId, domainToUpdate, updateDomainConfigParams);
[inline-code-end]
