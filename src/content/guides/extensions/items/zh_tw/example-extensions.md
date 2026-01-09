在 FastComments，我們使用相同的 API 自行撰寫擴充功能。您可以在下列端點查看這些擴充功能的未壓縮程式碼：

#### Dark Mode

當偵測到頁面為「深色」時，深色模式擴充功能會被有條件地載入。此擴充功能僅向評論元件加入一些 CSS。如此一來，當不需要時我們就不用載入深色模式的 CSS。

https://fastcomments.com/js/comment-ui/extensions/comment-ui.dark.extension.js

#### Moderation

當目前使用者為版主時，我們會使用審核擴充功能。

這是一個很好的範例，示範如何新增基於點擊的事件監聽器、發出 API 請求，以及新增項目到評論選單和評論區域。

https://fastcomments.com/js/comment-ui/extensions/comment-ui.moderation.extension.js

#### Live Chat

即時聊天擴充功能（搭配其他設定與樣式）會將評論元件轉變成即時聊天組件。

https://fastcomments.com/js/comment-ui/extensions/live-chat.extension.js