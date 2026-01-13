---
對於 SSO，需要針對通知考慮下列設定：

- 使用者是否已選擇接收通知。
  - 可在 `SSOUser` 物件中將 `optedInNotifications` 標誌設為 `true` 或 `false` 來完成該設定。
  - 可透過 API 設定。
  - 另外，如果您在 payload 中傳入此標誌的值，當使用者載入留言串時該值會自動更新。
- 使用者是否已選擇接收 **訂閱** 通知。
  - 可在 `SSOUser` 物件中將 `optedInSubscriptionNotifications` 標誌設為 `true` 或 `false` 來完成該設定。
  - 可透過 API 設定。
  - 另外，如果您在 payload 中傳入此標誌的值，當使用者載入留言串時該值會自動更新。
- 定義他們的電子郵件。
  - 若未提供，便無法傳送基於電子郵件的通知。
- 是否在電子郵件中停用取消訂閱連結。
  - 可在 `Tenant` 物件中透過 `disableUnsubscribeLinks` 標誌來完成。
  - 可透過 API 設定。
- 是否使用自訂的取消訂閱連結。
  - 可在 `DomainConfig` 物件的 `footerUnsubscribeURL` 屬性中設定。
  - 可透過 API 設定。
  - 您也可能想在相同物件中透過 `emailHeaders` 設定相關的取消訂閱標頭。

---