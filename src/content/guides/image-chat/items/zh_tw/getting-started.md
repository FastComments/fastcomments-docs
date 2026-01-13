### 使用情境

Image Chat 非常適合用於設計回饋，團隊需要就 mockup 或截圖中的特定元素進行討論。產品評論網站可以讓客戶就產品照片中可見的特定功能進行討論。教育平台可以用它來討論圖表、地圖或科學圖片。相片畫廊可以透過針對特定位置的評論變得互動化。房地產網站可以讓觀眾針對房屋照片中可見的特定特色進行討論。

### 快速開始

開始使用 Image Chat 很簡單。你需要 FastComments Image Chat 腳本、一個包含圖片的 image 元素或容器，還有一個包含你的 Tenant ID 的設定物件。

### 安裝

將 Image Chat 腳本新增到你的頁面：

[inline-code-attrs-start title = '載入 Image Chat 腳本'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### 基本實作

以下是一個最小範例：

[inline-code-attrs-start title = 'Image Chat 基本實作'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- 你的圖片 -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- 載入 Image Chat 腳本 -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- 初始化 Image Chat -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

如果尚未替換，請將 'demo' 替換為你的實際 FastComments Tenant ID，你可以在你的 [FastComments dashboard](https://fastcomments.com/auth/my-account) 找到它。

### 運作方式

初始化後，使用者可以在圖片的任意位置點擊。當發生點擊時，該位置會顯示一個視覺上的方形標記並開啟聊天視窗。其他使用者可以看到所有標記並點擊它們以檢視或參與那些討論。所有討論會在所有訪客之間即時同步。

此 widget 使用百分比定位，因此即使圖片大小改變或在不同螢幕尺寸上檢視，標記仍會保持在正確位置。

### 線上示範

你可以在我們的 [線上示範頁面](https://fastcomments.com/product/image-chat) 見到 Image Chat 的實際運作。

### 下一步

在基本功能運作後，你可以在 設定選項 指南 中自訂外觀和行為。查看 響應式設計 指南 以了解百分比定位的運作方式。於 自訂 指南 中了解樣式與深色模式支援。欲進行進階整合，請參閱 API 參考。

### 前端函式庫

所有 FastComments 的前端函式庫（react、vue、angular 等）都包含 Image Chat。