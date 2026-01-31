## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Response

Returns: [`GetUser200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_user Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetUserParams = GetUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user_98765".to_string(),
        include_profile: Some(true),
    };
    let user_response: GetUser200Response = get_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
