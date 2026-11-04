### 快速入門

開始使用 Collab Chat 非常簡單。您需要 FastComments Collab Chat 腳本、一個包含您想要標註文字的 HTML 元素，以及一個包含您的 Tenant ID 的設定物件。

### 安裝

將 Collab Chat 腳本加入您的頁面：

[inline-code-attrs-start title = '載入 Collab Chat 腳本'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### 基本實作

以下是一個最簡範例：

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

如果尚未設定，請將 `'demo'` 替換為您實際的 FastComments Tenant ID，您可以在 [FastComments 儀表板](https://fastcomments.com/auth/my-account/api-secret) 中找到它。

### 運作原理

初始化後，使用者可以選取目標元素內的任何文字。經過短暫延遲（桌面上為 3.5 秒）後，會出現一個提示，讓他們開始討論。當討論建立後，文字上會出現視覺化醒目提示。其他使用者可以將滑鼠懸停在醒目提示上或點擊它，以檢視並參與討論。所有討論會在所有訪客之間即時同步。

### 線上示範

您可以在我們的[線上示範頁面](https://fastcomments.com/product/collab-chat)上看到 Collab Chat 的實際運作。

### 後續步驟

現在您已經掌握了基本用法，可以在 Configuration Options 指南中自訂外觀與行為。請查看 Text Selection Behavior 指南以了解文字選取的運作方式。在 Customization 指南中了解樣式與深色模式支援。如需進階整合，請探索 API Reference。

### 前端函式庫

所有 FastComments 前端函式庫（react、vue、angular 等）都支援 Collab Chat。