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
async fn fetch_user_reacts() -> Result<GetUserReactsPublic200Response, Error> {
    let params: GetUserReactsPublicParams = GetUserReactsPublicParams {
        tenant_id: String::from("acme-corp-tenant"),
        post_ids: Some(vec![
            String::from("news/article-2025-11-21"),
            String::from("blog/product-release-2025"),
        ]),
        sso: Some(String::from("sso_user_abcd1234")),
    };
    let response: GetUserReactsPublic200Response = get_user_reacts_public(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
