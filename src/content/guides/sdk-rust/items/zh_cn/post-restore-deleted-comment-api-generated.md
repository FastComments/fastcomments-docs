## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| comment_id | String | 是 |  |
| sso | String | 否 |  |

## 响应

返回: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 示例

[inline-code-attrs-start title = 'post_restore_deleted_comment 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = PostRestoreDeletedCommentParams {
        comment_id: String::from("news/article-2024-06-19/comment-8932"),
        sso: Some(String::from("user-session-9f8e7d")),
    };
    let response: ApiEmptyResponse = post_restore_deleted_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---