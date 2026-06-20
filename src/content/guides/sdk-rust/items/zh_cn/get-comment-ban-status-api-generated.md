## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| comment_id | String | 是 |  |
| sso | String | 否 |  |

## 响应

返回：[`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_ban_status_response.rs)

## 示例

[inline-code-attrs-start title = 'get_comment_ban_status 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetCommentBanStatusParams = GetCommentBanStatusParams {
        comment_id: String::from("cmt-9f8b7a6e-4d3c-11ee-8c99-0242ac120002"),
        sso: Some(String::from("acme-corp-tenant")),
    };
    let response: GetCommentBanStatusResponse = get_comment_ban_status(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---