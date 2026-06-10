#### Skin: Erebus
![主題: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Skin: Default
![主題: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Native WYSIWYG Editor with Image Support!
![原生 WYSIWYG 編輯器，支援圖片](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Rich Text Editor

此函式庫使用 [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) 來進行富文字編輯，提供強大的所見即所得編輯體驗。相同的編輯器驅動 iOS、Android 以及透過 `react-native-web` 的網頁，因此在所有平台上只需一個實作就能讓作者行為一致。

`react-native-enriched` 在原生端需要 React Native New Architecture (Fabric)，並且需要能解析套件 `exports` 條件的打包器（Metro 與套件 exports / RN 0.72+）。網頁支援目前仍屬實驗性質。

### Configuration Options

此函式庫旨在支援與網頁實作相同，所有在 [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) 中定義的設定選項。

### FastComments Concepts

要開始使用，主要需要了解的概念是 `tenantId` 與 `urlId`。`tenantId` 是您在 FastComments.com 的帳戶識別。`urlId` 則是評論串會被綁定的位置。這可以是一個頁面 URL、產品 id、文章 id 等等。

### User Notifications

FastComments 支援 [多種情境](https://docs.fastcomments.com/guide-notifications.html) 的通知。通知可被設定，能在全域或單一通知/評論層級選擇退訂，並支援頁面級訂閱，使使用者可以訂閱特定頁面或文章的討論串。

例如，可以使用 Secure SSO 來驗證使用者，然後定期輪詢未讀通知並將它們推送給使用者。

參見 [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) 了解如何取得並轉譯使用者的未讀通知。

### Gif Browser

預設情況下，未啟用圖片或 gif 選擇功能。請參見 [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) 了解如何支援圖片與 gif 的上傳。此函式庫提供一個會將搜尋與圖片匿名化的 Gif 瀏覽器，您只需要使用它即可。

### Performance

如果您發現任何效能問題，請開一個議題並附上可重現的範例與所使用的裝置。效能是所有 FastComments 函式庫的首要考量。