### Temel Örnek

Collab Chat'i kullanmanın en basit yolu tek bir içerik kapsayıcısını hedeflemektir. Bu örnek, bir makalede metin açıklamalarını nasıl etkinleştireceğinizi gösterir:

[inline-code-attrs-start title = 'Temel Collab Chat Örneği'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Özel URL Kimliği ile Örnek (Kitap Sayfası, Makale, vb.)

Varsayılan olarak, Collab Chat konuşmaları tanımlamak için sayfa URL'sini, öğe yolunu ve metin aralığını kullanır. Konuşmaların nasıl gruplanacağı üzerinde daha fazla kontrol sahibi olmak istiyorsanız özel bir `urlId` sağlayabilirsiniz:

[inline-code-attrs-start title = 'Özel URL Kimliği ile Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    FastCommentsCollabChat(document.getElementById('article-content'), {
        tenantId: 'demo',
        urlId: 'book-one-page-2'
    });
</script>
[inline-code-end]

URL yapınız değişse bile aynı konuşmaları korumak istiyorsanız veya aynı konuşma açıklamalarını birden fazla sayfada paylaşmak istiyorsanız bu kullanışlıdır.

### Karanlık Mod ile Örnek

Sitenizin koyu bir arka planı varsa, sohbet UI'sinin doğru görünmesini sağlamak için karanlık mod desteğini etkinleştirin:

[inline-code-attrs-start title = 'Karanlık Mod ile Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Üst Çubuk Devre Dışı Örneği

Varsayılan olarak, Collab Chat kullanıcı sayısı ve tartışma sayısı ile bir üst çubuk gösterir. Bunu devre dışı bırakabilirsiniz:

[inline-code-attrs-start title = 'Üst Çubuk Devre Dışı Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Yorum Sayısı Geri Çağırması ile Örnek

Yorum eklendiğinde veya güncellendiğinde `commentCountUpdated` geri çağırmasını kullanarak takip edebilirsiniz:

[inline-code-attrs-start title = 'Yorum Sayısı Geri Çağırması ile Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### Birden Çok Bölümle Örnek

Sayfanızdaki birden çok ayrı bölümde Collab Chat başlatabilirsiniz. Her bölümün kendi bağımsız açıklamaları olur:

[inline-code-attrs-start title = 'Birden Çok Bölümde Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    // Giriş bölümü için başlat
    FastCommentsCollabChat(document.getElementById('intro-section'), {
        tenantId: 'demo',
        urlId: 'my-article-intro'
    });

    // Ana bölüm için başlat
    FastCommentsCollabChat(document.getElementById('main-section'), {
        tenantId: 'demo',
        urlId: 'my-article-main'
    });
</script>
[inline-code-end]

---