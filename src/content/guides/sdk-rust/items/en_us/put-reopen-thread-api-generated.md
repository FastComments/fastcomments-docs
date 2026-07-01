## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| sso | String | No |  |

## Response

Returns: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Example

[inline-code-attrs-start title = 'put_reopen_thread Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn reopen_thread_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = PutReopenThreadParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article-123".to_string(),
        sso: Some("user-42".to_string()),
    };
    let _response: ApiEmptyResponse = put_reopen_thread(configuration, params).await?;
    Ok(())
}
[inline-code-end]
