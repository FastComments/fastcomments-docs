## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| block_from_comment_params | models::BlockFromCommentParams | Yes |  |
| user_id | String | No |  |
| anon_user_id | String | No |  |

## 回應

回傳：[`BlockSuccess`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/block_success.rs)

## 範例

[inline-code-attrs-start title = 'block_user_from_comment 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = BlockUserFromCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-9876".to_string(),
        block_from_comment_params: models::BlockFromCommentParams {
            reason: "spam".to_string(),
        },
        user_id: Some("user-42".to_string()),
        anon_user_id: None,
    };
    let _result: BlockSuccess = block_user_from_comment(configuration, params).await?;
    Ok(())
}
[inline-code-end]