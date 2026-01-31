## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |

## Response

Returns: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_definitions_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_email_template_definitions Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<GetEmailTemplateDefinitions200Response, Error> {
    let params: GetEmailTemplateDefinitionsParams = GetEmailTemplateDefinitionsParams {
        tenant_id: "acme-corp-tenant".to_string(),
    };
    let response: GetEmailTemplateDefinitions200Response =
        get_email_template_definitions(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
