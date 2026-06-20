## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| comment_id | String | 是 |  |
| include_by_user_id_and_email | bool | 否 |  |
| include_by_ip | bool | 否 |  |
| include_by_email_domain | bool | 否 |  |
| sso | String | 否 |  |

## 回應

回傳: [`PreBanSummary`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/pre_ban_summary.rs)

## 範例

[inline-code-attrs-start title = 'get_pre_ban_summary 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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