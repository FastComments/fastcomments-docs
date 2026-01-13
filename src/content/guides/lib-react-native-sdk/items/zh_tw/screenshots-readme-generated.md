#### Skin: Erebus
![Skin: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Skin: Default
![Skin: Default](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Native WYSIWYG Editor with Image Support!
![Native WYSIWYG Editor with Image Support](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Rich Text Editor

此函式庫使用 10tap editor 提供富文字編輯功能，帶來強大的 WYSIWYG 編輯體驗。

### Configuration Options

此函式庫旨在支援 [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) 中定義的所有設定選項，就像網頁實作一樣。

### FastComments Concepts

要開始使用，主要需要了解的概念是 `tenantId` 和 `urlId`。`tenantId` 是您在 FastComments.com 的帳戶識別。`urlId` 則是評論串綁定的地方。這可以是頁面網址、產品編號、文章編號等。

### User Notifications

FastComments 支援多種通知情境，詳見 [多種情境](https://docs.fastcomments.com/guide-notifications.html)。通知是可配置的，可於全域或單一通知/評論層級選擇退出，並支援頁面層級訂閱，讓使用者可以訂閱特定頁面或文章的討論串。

例如，可以使用 Secure SSO 驗證使用者，然後定期輪詢未讀通知並推送給使用者。

請參閱 [範例 AppNotificationsSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) 了解如何取得並翻譯未讀使用者通知。

### Gif Browser

預設情況下，不會啟用任何圖片或 GIF 選取功能。請參閱 [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) 了解如何支援圖片與 GIF 上傳。此函式庫提供一個 Gif Browser，會對搜尋及提供的圖片進行匿名化處理，您只需使用它即可。

### Performance

如果您發現任何效能問題，請開一個 issue 並提供可重現的範例（包含所使用的裝置），我們會查看。效能是所有 FastComments 函式庫的首要考量。