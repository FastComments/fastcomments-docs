FastComments is a localized platform. All of our widgets, emails, and notifications are localized.

在地化意味著我們會根據使用者的位置
和偏好語言，顯示不同的語言與格式。我們是依據使用者瀏覽器提供的資訊來判定這些設定。

We can customize the text in the email by going to the `Translations` tab, selecting a `Locale`
並編輯文字。從預設值變更的文字會在使用者介面中以外框表示。你可以
在不同語系之間切換並在最後儲存，而不會遺失變更。

Localized text is accessed via the `TEXT` object, for example: `<%= TEXT.INTRO %>`.

### SSO 注意事項

For SSO integrations, if `locale` is not specified, it will be updated every time the user
以不同語系存取評論小工具時，`locale` 都會被更新。這表示他們的語言偏好
會自動更新，未來寄出的電子郵件也會以該語系發送。

This can also be set manually by providing `locale` in the SSO payload.

---