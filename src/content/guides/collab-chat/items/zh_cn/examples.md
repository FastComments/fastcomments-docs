### 基本示例

使用 Collab Chat 的最简单方法是针对单个内容容器。此示例演示如何在文章上启用文本注释：

[inline-code-attrs-start title = '基本 Collab Chat 示例'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <div id="article-content" style="min-height: 500px">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

### 使用自定义 URL ID 的示例（按书页、文章等）

默认情况下，Collab Chat 使用页面 URL 与元素路径和文本范围的组合来识别会话。您可以提供自定义的 `urlId` 来更好地控制对话如何分组：

[inline-code-attrs-start title = '带自定义 URL ID 的 Collab Chat 示例'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    FastCommentsCollabChat(document.getElementById('article-content'), {
        tenantId: 'demo',
        urlId: 'book-one-page-2'
    });
</script>
[inline-code-end]

如果您的 URL 结构发生更改但希望保留相同的会话，或者希望在多个页面之间共享相同的会话注释，这会很有用。

### 深色模式示例

如果您的网站有深色背景，请启用深色模式支持以确保聊天 UI 显示正确：

[inline-code-attrs-start title = '带深色模式的 Collab Chat 示例'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat - Dark Mode</title>
    <style>
        body {
            background-color: #1a1a1a !important;
            color: #e0e0e0 !important;
            font-family: system-ui, -apple-system, sans-serif;
            padding: 20px;
            margin: 0;
        }
        #article-content {
            max-width: 800px;
            margin: 0 auto;
            background-color: #2d2d2d;
            padding: 20px;
            border-radius: 8px;
        }
        h1 {
            color: #ffffff !important;
        }
        p {
            color: #e0e0e0 !important;
            line-height: 1.6;
        }
        .fastcomments-collab-chat-top-bar {
            background-color: #2d2d2d !important;
            color: #e0e0e0 !important;
            border-bottom: 1px solid #444 !important;
        }
    </style>
</head>
<body>
    <div id="article-content" style="min-height: 500px">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo',
            hasDarkBackground: true
        });
    </script>
</body>
</html>
[inline-code-end]

### 禁用顶部栏的示例

默认情况下，Collab Chat 会显示一个包含用户计数和讨论计数的顶部栏。您可以将其禁用：

[inline-code-attrs-start title = '禁用顶部栏的 Collab Chat 示例'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat - No Top Bar</title>
</head>
<body>
    <div id="article-content" style="min-height: 500px">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo',
            topBarTarget: null
        });
    </script>
</body>
</html>
[inline-code-end]

### 带评论计数回调的示例

您可以使用 `commentCountUpdated` 回调跟踪评论何时被添加或更新：

[inline-code-attrs-start title = '带评论计数回调的 Collab Chat 示例'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    FastCommentsCollabChat(document.getElementById('article-content'), {
        tenantId: 'demo',
        commentCountUpdated: function(count) {
            console.log('Total comments:', count);
            document.getElementById('comment-badge').textContent = count;
        }
    });
</script>
[inline-code-end]

### 带多个部分的示例

您可以在页面的多个独立部分上初始化 Collab Chat。每个部分将拥有自己的独立注释：

[inline-code-attrs-start title = '在多个部分使用 Collab Chat 示例'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div id="intro-section">
    <h2>Introduction</h2>
    <p>Content for the introduction...</p>
</div>

<div id="main-section">
    <h2>Main Content</h2>
    <p>Content for the main article...</p>
</div>

<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    // 在介绍部分初始化
    FastCommentsCollabChat(document.getElementById('intro-section'), {
        tenantId: 'demo',
        urlId: 'my-article-intro'
    });

    // 在主内容部分初始化
    FastCommentsCollabChat(document.getElementById('main-section'), {
        tenantId: 'demo',
        urlId: 'my-article-main'
    });
</script>
[inline-code-end]