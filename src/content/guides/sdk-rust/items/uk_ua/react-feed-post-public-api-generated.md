## Parameters

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| post_id | String | Так |  |
| react_body_params | models::ReactBodyParams | Так |  |
| is_undo | bool | Ні |  |
| broadcast_id | String | Ні |  |
| sso | String | Ні |  |

## Response

Повертає: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/react_feed_post_public_200_response.rs)