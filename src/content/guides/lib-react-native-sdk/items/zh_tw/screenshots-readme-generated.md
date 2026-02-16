#### 佈景主題: Erebus
![佈景主題: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### 佈景主題: Default
![佈景主題: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### 原生 WYSIWYG 編輯器，支援圖片！
![原生 WYSIWYG 編輯器，支援圖片](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### 富文本編輯器

此函式庫使用 10tap 編輯器來提供富文本編輯功能，帶來強大的 WYSIWYG 編輯體驗。

### 設定選項

此函式庫旨在像網頁實作一樣，支援 [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) 中定義的所有設定選項。

### FastComments 概念

開始時需要注意的主要概念是 `tenantId` 和 `urlId`。`tenantId` 是你在 FastComments.com 的帳戶識別碼。`urlId` 則是評論串要綁定的位置。這可以是頁面 URL、產品 id、文章 id，等等。

### 使用者通知

FastComments 支援 [多種情境](https://docs.fastcomments.com/guide-notifications.html) 的通知。通知可配置，使用者可以在全域或特定通知/評論層級選擇退出，並且支援頁面層級的訂閱，讓使用者可以訂閱特定頁面或文章的討論串。

例如，可以使用 Secure SSO 來驗證使用者，然後定期輪詢未讀通知並推送給使用者。

請參閱 [範例 AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) 以了解如何取得並處理未讀的使用者通知。

### Gif Browser

預設情況下，未啟用任何圖片或 GIF 選取。請參閱 [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) 了解如何支援圖片和 gif 上傳。此函式庫提供一個會匿名化搜尋和圖像的 Gif Browser，你只需使用它即可。

### 效能

如果你發現任何效能問題，請開一張工單並附上可重現的範例及所使用的裝置。效能是所有 FastComments 函式庫的首要考量。