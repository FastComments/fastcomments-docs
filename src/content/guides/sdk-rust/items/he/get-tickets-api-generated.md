## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| user_id | String | לא |  |
| state | f64 | לא |  |
| skip | f64 | לא |  |
| limit | f64 | לא |  |

## תגובה

מחזיר: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tickets_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_tickets'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_tickets() -> Result<(), Error> {
    let params: GetTicketsParams = GetTicketsParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        user_id: Some("user-9876".to_owned()),
        state: Some(1.0),
        skip: Some(0.0),
        limit: Some(25.0),
    };
    let tickets: GetTickets200Response = get_tickets(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---