## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| create_comment_params | models::CreateCommentParams | 是 |  |
| is_live | bool | 否 |  |
| do_spam_check | bool | 否 |  |
| send_emails | bool | 否 |  |
| populate_notifications | bool | 否 |  |

## 回應

回傳: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/save_comment_200_response.rs)

---