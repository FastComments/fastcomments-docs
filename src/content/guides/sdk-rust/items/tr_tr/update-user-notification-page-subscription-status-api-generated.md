---
Bir sayfa için bildirimleri etkinleştirin veya devre dışı bırakın. Kullanıcılar bir sayfaya abone olduğunda, bildirimler oluşturulur
yeni kök yorumlar için, ve ayrıca

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| url_id | String | Evet |  |
| url | String | Evet |  |
| page_title | String | Evet |  |
| subscribed_or_unsubscribed | String | Evet |  |
| sso | String | Hayır |  |

## Yanıt

Geri döner: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

---