### דוגמה בסיסית

הדרך הפשוטה ביותר להשתמש ב-Collab Chat היא למקד מיכל תוכן יחיד. הדוגמה הזו מראה כיצד להפעיל הערות טקסט במאמר:

[inline-code-attrs-start title = 'דוגמה בסיסית של Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### דוגמה עם מזהה URL מותאם (לעמוד בספר, מאמר וכו')

כברירת מחדל, Collab Chat משתמש ב-URL של הדף בשילוב עם נתיב האלמנט וטווח הטקסט כדי לזהות שיחות. תוכלו לספק `urlId` מותאם כדי לקבל שליטה רבה יותר על האופן שבו השיחות מקובצות:

[inline-code-attrs-start title = 'Collab Chat עם מזהה URL מותאם'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    FastCommentsCollabChat(document.getElementById('article-content'), {
        tenantId: 'demo',
        urlId: 'book-one-page-2'
    });
</script>
[inline-code-end]

זה שימושי אם מבנה ה-URL שלכם משתנה אבל אתם רוצים לשמור על אותן שיחות, או אם אתם רוצים לשתף את אותן ההערות של השיחה בין עמודים מרובים.

### דוגמה עם מצב כהה

אם לאתר שלכם יש רקע כהה, אפשרו תמיכה במצב כהה כדי להבטיח שממשק הצ'אט ייראה תקין:

[inline-code-attrs-start title = 'Collab Chat עם מצב כהה'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### דוגמה עם הסרגל העליון מושבת

כברירת מחדל, Collab Chat מציג סרגל עליון עם מונה משתמשים ומונה דיונים. ניתן להשבית אותו:

[inline-code-attrs-start title = 'Collab Chat עם הסרגל העליון מושבת'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### דוגמה עם קריאת חזרה לעדכון ספירת תגובות

ניתן לעקוב לאחר הוספה או עדכון של תגובות באמצעות פונקציית ה-`commentCountUpdated`:

[inline-code-attrs-start title = 'Collab Chat עם קריאת חזרה לספירת תגובות'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### דוגמה עם מקטעים מרובים

ניתן לאתחל את Collab Chat על מספר מקטעים נפרדים בדף שלכם. לכל מקטע יהיו ההערות העצמאיות שלו:

[inline-code-attrs-start title = 'Collab Chat על מספר מקטעים'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    // Initialize on intro section
    FastCommentsCollabChat(document.getElementById('intro-section'), {
        tenantId: 'demo',
        urlId: 'my-article-intro'
    });

    // Initialize on main section
    FastCommentsCollabChat(document.getElementById('main-section'), {
        tenantId: 'demo',
        urlId: 'my-article-main'
    });
</script>
[inline-code-end]

---