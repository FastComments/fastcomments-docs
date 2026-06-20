---
현재 페이지에 온라인 상태인 뷰어: 현재 웹소켓 세션이 해당 페이지에 구독되어 있는 사람들입니다.
익명 수인 anonCount + 총 수인 totalCount를 반환합니다 (룸 전체 구독자 수, 열거하지 않는 익명 뷰어 포함).

## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|------|
| tenant_id | String | 예 |  |
| url_id | String | 예 |  |
| after_name | String | 아니오 |  |
| after_user_id | String | 아니오 |  |

## 응답

반환: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## 예제

[inline-code-attrs-start title = 'get_online_users 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_online_users() -> Result<PageUsersOnlineResponse, Error> {
    let params: GetOnlineUsersParams = GetOnlineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/world/article-2026".to_string(),
        after_name: Some("jane.doe".to_string()),
        after_user_id: Some("user_98765".to_string()),
    };
    let response: PageUsersOnlineResponse = get_online_users(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---