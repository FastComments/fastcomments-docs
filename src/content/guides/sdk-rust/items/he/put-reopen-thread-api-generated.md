## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| url_id | String | כן |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-put_reopen_thread'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_reopen_thread() -> Result<(), Error> {
    let params: PutReopenThreadParams = PutReopenThreadParams {
        url_id: String::from("acme-corp/news/article-2026-06-19"),
        sso: Some(String::from("sso-token-9f8e7d6c")),
    };
    let response: ApiEmptyResponse = put_reopen_thread(configuration, params).await?;
    let _response = response;
    Ok(())
}
[inline-code-end]

---