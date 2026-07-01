## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|------------|------|
| tenant_id | String | Так |  |
| comment_id | String | Так |  |
| ban_email | bool | Ні |  |
| ban_email_domain | bool | Ні |  |
| ban_ip | bool | Ні |  |
| delete_all_users_comments | bool | Ні |  |
| banned_until | String | Ні |  |
| is_shadow_ban | bool | Ні |  |
| update_id | String | Ні |  |
| ban_reason | String | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/ban_user_from_comment_result.rs)

## Приклад

[inline-code-attrs-start title = 'post_ban_user_from_comment Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = PostBanUserFromCommentParams {
        tenant_id: "acme-corp".to_string(),
        comment_id: "cmt-12345".to_string(),
        ban_email: Some(true),
        ban_email_domain: Some(false),
        ban_ip: Some(true),
        delete_all_users_comments: Some(false),
        banned_until: Some("2024-12-31T23:59:59Z".to_string()),
        is_shadow_ban: Some(false),
        update_id: Some("upd-987".to_string()),
        ban_reason: Some("spam".to_string()),
        sso: Some("sso-provider".to_string()),
    };
    let _result: BanUserFromCommentResult = post_ban_user_from_comment(configuration, params).await?;
    Ok(())
}
[inline-code-end]