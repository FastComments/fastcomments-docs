## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |

## 响应

返回: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_response.rs)

## 示例

[inline-code-attrs-start title = 'get_votes 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_votes(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetVotesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        limit: Some(100),
    };
    let _response: GetVotesResponse = get_votes(configuration, params).await?;
    Ok(())
}
[inline-code-end]