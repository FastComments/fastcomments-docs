req
tenantId
urlId
userIdWS

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| url_id | String | כן |  |
| user_id_ws | String | כן |  |
| start_time | i64 | כן |  |
| end_time | i64 | כן |  |

## תגובה

מחזיר: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_event_log'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetEventLogParams = GetEventLogParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article-2023-01-01".to_string(),
        user_id_ws: "user-12345-ws".to_string(),
        start_time: 1672531200i64,
        end_time: 1672617599i64,
        include_details: Some(true),
    };
    let response: GetEventLog200Response = get_event_log(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---