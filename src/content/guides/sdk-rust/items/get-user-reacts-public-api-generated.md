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
        tenant_id: "acme-corp-tenant".to_string(),
        post_ids: Some(vec![
            "news/article/123".to_string(),
            "blog/post/987".to_string(),
        ]),
        sso: Some("user-42@example.com".to_string()),
    };
    let response: GetUserReactsPublic200Response =
        get_user_reacts_public(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
