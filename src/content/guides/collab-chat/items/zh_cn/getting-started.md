### 快速开始

开始使用 Collab Chat 非常简单。你需要 FastComments Collab Chat 脚本、一个包含要标注文本的 HTML 元素，以及一个包含你的 Tenant ID 的配置对象。

### 安装

将 Collab Chat 脚本添加到你的页面：

[inline-code-attrs-start title = '加载 Collab Chat 脚本'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### 基本实现

以下是一个最简示例：

[inline-code-attrs-start title = '基本 Collab Chat 实现'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Your content container -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Load the Collab Chat script -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Initialize Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

如果尚未替换，请将 `'demo'` 替换为你实际的 FastComments Tenant ID，你可以在 [FastComments 控制台](https://fastcomments.com/auth/my-account/api-secret) 中找到它。

### 工作原理

初始化后，用户可以选择目标元素内的任何文本。经过短暂延迟（桌面端为 3.5 秒）后，会出现一个提示，允许他们发起讨论。创建讨论后，文本上会出现视觉高亮。其他用户可以悬停或点击高亮来查看并参与讨论。所有讨论会在所有访客之间实时同步。

### 在线演示

你可以在我们的[在线演示页面](https://fastcomments.com/product/collab-chat)上查看 Collab Chat 的实际效果。

### 后续步骤

现在你已经掌握了基础知识，可以在“配置选项”指南中自定义外观和行为。请查看“文本选择行为”指南以了解文本选择的工作方式。在“自定义”指南中了解样式和深色模式支持。对于高级集成，请探索 API 参考。

### 前端库

所有 FastComments 前端库（react、vue、angular 等）都支持 Collab Chat。