## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| comment_id | String | Так |  |
| include_by_user_id_and_email | bool | Ні |  |
| include_by_ip | bool | Ні |  |
| include_by_email_domain | bool | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`PreBanSummary`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/pre_ban_summary.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_pre_ban_summary'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetPreBanSummaryParams = GetPreBanSummaryParams {
    comment_id: String::from("news/article-9876-comment-42"),
    include_by_user_id_and_email: Some(true),
    include_by_ip: Some(false),
    include_by_email_domain: Some(true),
    sso: Some(String::from("sso-acme-corp-2026")),
};
let pre_ban_summary: PreBanSummary = get_pre_ban_summary(configuration, params).await?;
[inline-code-end]

---