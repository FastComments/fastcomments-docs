## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| body | models::PickApiCommentPeriodUpdatableCommentFields | 是 |  |
| context_user_id | String | 否 |  |
| do_spam_check | bool | 否 |  |
| is_live | bool | 否 |  |

## 回應

回傳：[`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

---