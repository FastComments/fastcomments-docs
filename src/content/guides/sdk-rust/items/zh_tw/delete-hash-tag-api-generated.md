## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tag | String | 是 |  |
| tenant_id | String | 否 |  |
| delete_hash_tag_request | models::DeleteHashTagRequest | 否 |  |

## 回應

回傳：[`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 範例

[inline-code-attrs-start title = 'delete_hash_tag 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_tag(configuration: &configuration::Configuration) -> Result<FlagCommentPublic200Response, Error> {
    let params = DeleteHashTagParams {
        tag: "news/world-climate".to_owned(),
        tenant_id: Some("acme-corp-tenant".to_owned()),
        delete_hash_tag_request: None,
    };
    let response: FlagCommentPublic200Response = delete_hash_tag(configuration, params).await?;
    Ok(response)
}
[inline-code-end]