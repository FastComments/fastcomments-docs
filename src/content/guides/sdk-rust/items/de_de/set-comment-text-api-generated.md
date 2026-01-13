---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| broadcast_id | String | Ja |  |
| comment_text_update_request | models::CommentTextUpdateRequest | Ja |  |
| edit_key | String | Nein |  |
| sso | String | Nein |  |

## Antwort

Gibt zurück: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_text_200_response.rs)

---