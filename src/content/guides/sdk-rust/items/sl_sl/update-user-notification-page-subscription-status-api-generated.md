Omogočite ali onemogočite obvestila za stran. Ko so uporabniki naročeni na stran, se ustvarijo obvestila za nove glavne komentarje, in tudi

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| url_id | String | Da |  |
| url | String | Da |  |
| page_title | String | Da |  |
| subscribed_or_unsubscribed | String | Da |  |
| sso | String | Ne |  |

## Odgovor

Vrača: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)