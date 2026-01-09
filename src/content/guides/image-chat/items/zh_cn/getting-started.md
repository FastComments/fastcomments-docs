### 使用场景

Image Chat 非常适合用于需要就设计稿或屏幕截图中的特定元素进行讨论的设计反馈。产品评价网站可以让客户讨论产品照片中可见的特定功能。教育平台可以用来讨论图表、地图或科学图像。照片画廊可以通过基于位置的评论变得互动。房地产网站可以让查看者讨论房源照片中可见的特定特征。

### 快速开始

开始使用 Image Chat 很简单。您需要 FastComments Image Chat 脚本、一个包含图像的图像元素或容器，以及包含您的 Tenant ID 的配置对象。

### 安装

将 Image Chat 脚本添加到您的页面：

[inline-code-attrs-start title = '加载 Image Chat 脚本'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### 基本实现

下面是一个最小示例：

[inline-code-attrs-start title = 'Image Chat 基本实现'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- 您的图片 -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- 加载 Image Chat 脚本 -->
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

将 `'demo'` 替换为您实际的 FastComments Tenant ID（如果尚未设置），您可以在 [FastComments 仪表板](https://fastcomments.com/auth/my-account) 中找到它。

### 工作原理

初始化后，用户可以在图像的任意位置点击。发生点击时，会在该位置显示一个可视的方形标记并打开聊天窗口。其他用户可以看到所有标记并点击它们以查看或参与这些讨论。所有讨论会在所有访问者之间实时同步。

该小部件使用基于百分比的定位，因此即使图像调整大小或在不同屏幕尺寸上查看，标记也会保持在正确的位置。

### 在线演示

您可以在我们的 [在线演示页面](https://fastcomments.com/product/image-chat) 上查看 Image Chat 的实际效果。

### 后续步骤

现在您已经掌握了基础用法，您可以在“配置选项”指南中自定义外观和行为。查看“响应式设计”指南以了解基于百分比的定位如何工作。在“自定义”指南中了解样式和暗色模式支持。对于高级集成，请参考 API 参考。

### 前端库

所有 FastComments 的前端库（react、vue、angular 等）都包含 Image Chat。