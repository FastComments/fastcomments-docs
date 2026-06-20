## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| comment_id | String | Ja |  |
| include_by_user_id_and_email | bool | Nein |  |
| include_by_ip | bool | Nein |  |
| include_by_email_domain | bool | Nein |  |
| sso | String | Nein |  |

## Antwort

Gibt zurück: [`PreBanSummary`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/pre_ban_summary.rs)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für get_pre_ban_summary'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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