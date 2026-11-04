현재 온라인 상태가 아닌, 해당 페이지에서 이전에 댓글을 작성한 사용자들입니다. displayName 기준으로 정렬됩니다.
"멤버" 섹션을 렌더링할 때 /users/online을 모두 소진한 후 이 API를 사용하세요.
commenterName 기준 커서 페이지네이션: 서버는 부분 {tenantId, urlId, commenterName}
인덱스를 afterName부터 $gt를 통해 순회하며, $skip 비용이 없습니다.

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| url_id | String | 예 |  |
| after_name | String | 아니요 |  |
| after_user_id | String | 아니요 |  |

## 응답

반환값: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## 예제

[inline-code-attrs-start title = 'get_offline_users 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("alice".to_string()),
        after_user_id: Some("user-42".to_string()),
    };
    let _response = get_offline_users(config, params).await?;
    Ok(())
}
[inline-code-end]