## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| post_ids | Vec<String> | No |  |
| sso | String | No |  |

## Response

Returns: [`GetUserReactsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_reacts_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_user_reacts_public Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetUserReactsPublicParams = GetUserReactsPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_ids: Some(vec![
            "news/article-2026-01-12-breaking".to_string(),
            "blog/product-launch-2026".to_string(),
        ]),
        sso: Some("sso-jwt-3f2a1b9e".to_string()),
    };
    let response: GetUserReactsPublic200Response =
        get_user_reacts_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
