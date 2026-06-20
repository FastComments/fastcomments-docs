## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |

## 응답

반환: [`ApiGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_progress_response.rs)

## 예제

[inline-code-attrs-start title = 'get_user_badge_progress_by_id 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetUserBadgeProgressByIdParams = GetUserBadgeProgressByIdParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        id: "badge-gold-2026".to_owned(),
        user_id: Some("user-987".to_owned()),
    };
    let badge_progress: ApiGetUserBadgeProgressResponse =
        get_user_badge_progress_by_id(&configuration, params).await?;
    println!("{:#?}", badge_progress);
    Ok(())
}
[inline-code-end]

---