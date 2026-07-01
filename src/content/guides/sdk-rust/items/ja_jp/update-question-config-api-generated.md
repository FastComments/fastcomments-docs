## パラメータ

| 名前 | タイプ | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_question_config_body | models::UpdateQuestionConfigBody | Yes |  |

## レスポンス

戻り値: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 例

[inline-code-attrs-start title = 'update_question_config の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = UpdateQuestionConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
        update_question_config_body: UpdateQuestionConfigBody {
            title: "Breaking News".to_string(),
            is_active: true,
            custom_options: vec![
                QuestionConfigCustomOptionsInner {
                    key: "priority".to_string(),
                    value: "high".to_string(),
                },
            ],
            description: Some("Config for breaking news article".to_string()),
        },
    };
    update_question_config(&configuration, params).await?;
    Ok(())
}
[inline-code-end]