## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Response

Returns: [`GetUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_response.rs)

## Example

[inline-code-attrs-start title = 'get_user Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_user() -> Result<(), Error> {
    let params = GetUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-7b9a2c".to_string(),
        include_roles: Some(true),
    };
    let user: GetUserResponse = get_user(&configuration, params).await?;
    println!("{:#?}", user);
    Ok(())
}
[inline-code-end]
