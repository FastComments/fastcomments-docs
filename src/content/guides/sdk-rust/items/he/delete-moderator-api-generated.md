## פרמטרים

| שם | סוג | דרוש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| id | String | כן |  |
| send_email | String | לא |  |

## תגובה

מחזיר: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-delete_moderator'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteModeratorParams = DeleteModeratorParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("moderator-93b1f"),
        send_email: Some(String::from("moderator@acme-corp.com")),
    };
    let _response: ApiEmptyResponse = delete_moderator(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---