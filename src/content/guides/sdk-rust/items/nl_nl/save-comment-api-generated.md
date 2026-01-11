## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| create_comment_params | models::CreateCommentParams | Ja |  |
| is_live | bool | Nee |  |
| do_spam_check | bool | Nee |  |
| send_emails | bool | Nee |  |
| populate_notifications | bool | Nee |  |

## Response

Retourneert: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/save_comment_200_response.rs)