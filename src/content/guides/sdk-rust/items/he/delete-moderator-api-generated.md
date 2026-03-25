## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| id | String | כן |  |
| send_email | String | לא |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-delete_moderator'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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