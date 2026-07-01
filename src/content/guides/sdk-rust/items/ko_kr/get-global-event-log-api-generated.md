req
tenantId
urlId
userIdWS

## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|------|
| tenant_id | String | 예 |  |
| url_id | String | 예 |  |
| user_id_ws | String | 예 |  |
| start_time | i64 | 예 |  |
| end_time | i64 | 아니오 |  |

## 응답

반환: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_response.rs)

## 예시

[inline-code-attrs-start title = 'get_global_event_log 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetGlobalEventLogParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        user_id_ws: "user-12345".to_string(),
        start_time: 1_680_000_000,
        end_time: Some(1_680_864_000),
    };
    let _response = get_global_event_log(configuration, params).await?;
    Ok(())
}
[inline-code-end]