## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| send_email | String | 否 |  |

## 响应

返回：[`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 示例

[inline-code-attrs-start title = 'delete_moderator 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteModeratorParams = DeleteModeratorParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("moderator-9876"),
        send_email: Some(String::from("true")),
    };

    let response: FlagCommentPublic200Response = delete_moderator(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---