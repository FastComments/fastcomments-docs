## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| post_ids | Vec<String> | 否 |  |
| sso | String | 否 |  |

## 响应

返回：[`GetUserReactsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_reacts_public_200_response.rs)

## 示例

[inline-code-attrs-start title = 'get_user_reacts_public 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<GetUserReactsPublic200Response, Error> {
    let params: GetUserReactsPublicParams = GetUserReactsPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_ids: Some(vec![
            "news/article-123".to_string(),
            "blog/post-456".to_string(),
        ]),
        sso: Some("john.doe@acme.com".to_string()),
    };
    let response: GetUserReactsPublic200Response = get_user_reacts_public(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]