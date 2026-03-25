## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |
| block_from_comment_params | models::BlockFromCommentParams | 예 |  |
| user_id | String | 아니오 |  |
| anon_user_id | String | 아니오 |  |

## 응답

반환: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/block_from_comment_public_200_response.rs)

## 예제

[inline-code-attrs-start title = 'block_user_from_comment 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: BlockUserFromCommentParams = BlockUserFromCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-2026-03-25/comment-842".to_string(),
        block_from_comment_params: models::BlockFromCommentParams {
            reason: Some("Repeated promotional links".to_string()),
            duration_minutes: Some(7_200),
            notify_user: Some(true),
        },
        user_id: Some("user-9812".to_string()),
        anon_user_id: None,
    };
    let response: BlockFromCommentPublic200Response = block_user_from_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---