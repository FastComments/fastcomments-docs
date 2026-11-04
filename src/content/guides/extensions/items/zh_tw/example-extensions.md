在 FastComments，我們使用相同的 API 來撰寫自己的擴充功能。您可以在以下端點查看這些擴充功能的未壓縮程式碼：

#### 深色模式

深色模式擴充功能會在偵測到「深色」頁面時有條件地載入。此擴充功能只是為留言小工具新增一些 CSS。這樣一來，我們就不需要在不需要時載入深色模式的 CSS。

https://fastcomments.com/js/comment-ui/extensions/comment-ui.dark.extension.js

#### 審核

當目前使用者是審核員時，我們會使用審核擴充功能。

這是一個很好的範例，展示了如何新增基於點擊的事件監聽器、發送 API 請求，以及在留言選單和留言區域中新增內容。

https://fastcomments.com/js/comment-ui/extensions/comment-ui.moderation.extension.js

#### 即時聊天

即時聊天擴充功能（結合其他配置與樣式）可將留言小工具轉換為即時聊天元件。

https://fastcomments.com/js/comment-ui/extensions/live-chat.extension.js