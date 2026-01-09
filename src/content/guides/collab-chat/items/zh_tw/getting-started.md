### 快速開始

開始使用 Collab Chat 很簡單。您需要 FastComments Collab Chat 腳本、一個包含您想註解的文字的 HTML 元素，以及一個包含您的 Tenant ID 的設定物件。

### 安裝

將 Collab Chat 腳本加入您的頁面：

[inline-code-attrs-start title = '載入 Collab Chat 腳本'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### 基本實作

以下是一個最小範例：

[inline-code-attrs-start title = '基本 Collab Chat 實作'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- 您的內容容器 -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- 載入 Collab Chat 腳本 -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- 初始化 Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Replace `'demo'` with your actual FastComments Tenant ID if it's not already, which you can find in your [FastComments 儀表板](https://fastcomments.com/auth/my-account/api-secret).

### 運作方式

初始化後，使用者可以在目標元素內選取任意文字。經過短暫延遲（桌面版為 3.5 秒）後，會出現提示，讓他們開始討論。建立討論後，該文字會出現視覺上的高亮。其他使用者可以將滑鼠移到高亮上或點擊它以查看並參與討論。所有討論會在所有訪客之間即時同步。

### 線上示範

您可以在我們的 [線上示範頁面](https://fastcomments.com/product/collab-chat) 查看 Collab Chat 的實際示範。

### 下一步

既然您已完成基礎設置，您可以在「設定選項」指南中自訂外觀與行為。查看「文字選取行為」指南以了解文字選取的運作方式。在「自訂」指南中了解樣式和深色模式支援。若需進階整合，請參閱「API 參考」。

### 前端程式庫

所有 FastComments 的前端程式庫（react、vue、angular 等）都支援 Collab Chat。

---