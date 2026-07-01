Past commenters on the page who are NOT currently online. Sorted by displayName.  
현재 온라인 상태가 아닌 페이지의 과거 댓글 작성자들. displayName 기준으로 정렬됩니다.

Use this after exhausting /users/online to render a "Members" section.  
`/users/online`을 모두 사용한 후에 "Members" 섹션을 렌더링하기 위해 사용합니다.

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}
index from afterName forward via $gt, no $skip cost.  
commenterName에 대한 커서 페이지네이션: 서버가 {tenantId, urlId, commenterName} 부분 인덱스를 afterName 이후로 $gt를 이용해 순회하며, $skip 비용이 없습니다.

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| url_id | String | 예 |  |
| after_name | String | 아니오 |  |
| after_user_id | String | 아니오 |  |

## 응답

반환: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## 예시

[inline-code-attrs-start title = 'get_offline_users 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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