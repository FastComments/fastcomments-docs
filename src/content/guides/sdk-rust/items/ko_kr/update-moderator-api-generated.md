## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |
| update_moderator_body | models::UpdateModeratorBody | 예 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 예제

[inline-code-attrs-start title = 'update_moderator 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: UpdateModeratorParams = UpdateModeratorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "moderator-987".to_string(),
        update_moderator_body: models::UpdateModeratorBody {
            username: Some("jane.doe".to_string()),
            email: Some("jane.doe@acme.com".to_string()),
            role: Some("senior_moderator".to_string()),
            active: Some(true),
            notes: Some("Promoted after successful trial period".to_string()),
        },
    };
    let response: FlagCommentPublic200Response = update_moderator(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---