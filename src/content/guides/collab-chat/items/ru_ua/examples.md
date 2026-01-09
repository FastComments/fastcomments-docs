### Базовый пример

Самый простой способ использовать Collab Chat — нацелиться на один контейнер с содержимым. В этом примере показано, как включить текстовые аннотации в статье:

[inline-code-attrs-start title = 'Базовый пример Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Пример с пользовательским URL ID (для страницы книги, статьи и т.д.)

По умолчанию Collab Chat использует URL страницы в сочетании с путем к элементу и диапазоном текста для идентификации разговоров. Вы можете указать пользовательский `urlId`, чтобы иметь больший контроль над тем, как группируются разговоры:

[inline-code-attrs-start title = 'Collab Chat с пользовательским URL ID'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    FastCommentsCollabChat(document.getElementById('article-content'), {
        tenantId: 'demo',
        urlId: 'book-one-page-2'
    });
</script>
[inline-code-end]

Это полезно, если структура ваших URL меняется, но вы хотите сохранить те же разговоры, или если хотите разделять одни и те же аннотации разговоров на нескольких страницах.

### Пример с тёмным режимом

Если на вашем сайте тёмный фон, включите поддержку тёмного режима, чтобы интерфейс чата отображался корректно:

[inline-code-attrs-start title = 'Collab Chat с тёмным режимом'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Пример с отключённой верхней панелью

По умолчанию Collab Chat показывает верхнюю панель с количеством пользователей и количеством обсуждений. Вы можете отключить её:

[inline-code-attrs-start title = 'Collab Chat с отключённой верхней панелью'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Пример с колбэком обновления количества комментариев

Вы можете отслеживать добавление или обновление комментариев с помощью колбэка `commentCountUpdated`:

[inline-code-attrs-start title = 'Collab Chat с колбэком обновления количества комментариев'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### Пример с несколькими разделами

Вы можете инициализировать Collab Chat на нескольких отдельных разделах вашей страницы. Каждый раздел будет иметь свои независимые аннотации:

[inline-code-attrs-start title = 'Collab Chat на нескольких разделах'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    // Инициализация на секции intro
    FastCommentsCollabChat(document.getElementById('intro-section'), {
        tenantId: 'demo',
        urlId: 'my-article-intro'
    });

    // Инициализация на основной секции
    FastCommentsCollabChat(document.getElementById('main-section'), {
        tenantId: 'demo',
        urlId: 'my-article-main'
    });
</script>
[inline-code-end]

---