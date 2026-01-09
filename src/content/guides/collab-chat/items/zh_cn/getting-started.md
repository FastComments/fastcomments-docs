### 快速开始

开始使用 Collab Chat 很简单。您需要 FastComments Collab Chat 脚本、包含要注释文本的 HTML 元素，以及包含您的 Tenant ID 的配置对象。

### 安装

将 Collab Chat 脚本添加到您的页面：

[inline-code-attrs-start title = '加载 Collab Chat 脚本'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### 基本实现

下面是一个最小示例：

[inline-code-attrs-start title = '基本 Collab Chat 实现'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- 您的内容容器 -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- 加载 Collab Chat 脚本 -->
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

Replace `'demo'` with your actual FastComments Tenant ID if it's not already, which you can find in your [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

### 工作原理

初始化后，用户可以在目标元素内选择任意文本。短暂延迟后（桌面端为 3.5 秒），会出现提示，允许他们开始讨论。创建讨论后，文本上会出现视觉高亮。其他用户可以将鼠标悬停或点击该高亮以查看并参与讨论。所有讨论会在所有访问者之间实时同步。

### 在线演示

您可以在我们的[在线演示页面](https://fastcomments.com/product/collab-chat)查看 Collab Chat 的实际效果。

### 后续步骤

现在基础功能已就绪，您可以在配置选项指南中自定义外观和行为。查看文本选择行为指南以了解文本选择的工作方式。在自定义指南中了解样式和暗色模式支持。对于高级集成，请参阅 API 参考。

### 前端库

所有 FastComments 前端库（react、vue、angular 等）都包含 Collab Chat。

---