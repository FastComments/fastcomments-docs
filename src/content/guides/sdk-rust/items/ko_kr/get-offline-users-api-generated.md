---
페이지에 과거에 댓글을 남겼지만 현재 온라인이 아닌 사용자들입니다. displayName으로 정렬됩니다.
사용자 목록을 렌더링하기 위해 /users/online를 모두 조회한 후 "Members" 섹션을 렌더링하는 데 사용하세요.
commenterName에 대한 커서 페이지네이션: 서버는 부분 인덱스 {tenantId, urlId, commenterName}를 afterName 이후부터 $gt로 앞으로 탐색하며 $skip 비용이 들지 않습니다.

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| after_name | String | No |  |
| after_user_id | String | No |  |

## 응답

반환: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## 예제

[inline-code-attrs-start title = 'get_offline_users 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline_users() -> Result<PageUsersOfflineResponse, Error> {
    let params: GetOfflineUsersParams = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/world/today".to_string(),
        after_name: Some("jane.smith".to_string()),
        after_user_id: Some("user-1024".to_string()),
    };
    let response: PageUsersOfflineResponse = get_offline_users(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---