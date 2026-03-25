## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| user_id | String | 예 |  |

## 응답

반환값: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_badge_progress_by_id_200_response.rs)

## 예제

[inline-code-attrs-start title = 'get_user_badge_progress_by_user_id 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<GetUserBadgeProgressById200Response, Error> {
    let cfg: &configuration::Configuration = &configuration;
    let params: GetUserBadgeProgressByUserIdParams = GetUserBadgeProgressByUserIdParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: "journalist-9876".to_string(),
        include_inactive: Some(false),
        locale: Some("en-US".to_string()),
    };
    let response: GetUserBadgeProgressById200Response =
        get_user_badge_progress_by_user_id(cfg, params).await?;
    Ok(response)
}
[inline-code-end]

---