### Podstawowy przykład

Najprostszy sposób użycia Collab Chat to wskazanie jednego kontenera z treścią. Ten przykład pokazuje, jak włączyć adnotacje tekstu w artykule:

[inline-code-attrs-start title = 'Podstawowy przykład Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Przykład z niestandardowym identyfikatorem URL (na stronę książki, artykuł itp.)

Domyślnie Collab Chat używa adresu URL strony w połączeniu ze ścieżką elementu i zakresem tekstu do identyfikacji rozmów. Możesz podać niestandardowy `urlId`, aby mieć większą kontrolę nad tym, jak rozmowy są grupowane:

[inline-code-attrs-start title = 'Collab Chat z niestandardowym identyfikatorem URL'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    FastCommentsCollabChat(document.getElementById('article-content'), {
        tenantId: 'demo',
        urlId: 'book-one-page-2'
    });
</script>
[inline-code-end]

Jest to przydatne, jeśli struktura Twoich adresów URL się zmienia, ale chcesz zachować te same rozmowy, lub jeśli chcesz udostępniać te same adnotacje konwersacji na wielu stronach.

### Przykład z trybem ciemnym

Jeśli Twoja strona ma ciemne tło, włącz obsługę trybu ciemnego, aby interfejs czatu wyglądał poprawnie:

[inline-code-attrs-start title = 'Collab Chat z trybem ciemnym'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Przykład z wyłączonym paskiem górnym

Domyślnie Collab Chat pokazuje pasek górny z liczbą użytkowników i liczbą dyskusji. Możesz go wyłączyć:

[inline-code-attrs-start title = 'Collab Chat z wyłączonym paskiem górnym'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Przykład z callbackiem aktualizacji liczby komentarzy

Możesz śledzić dodawanie lub aktualizację komentarzy, używając callbacku `commentCountUpdated`:

[inline-code-attrs-start title = 'Collab Chat z callbackiem aktualizacji liczby komentarzy'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### Przykład z wieloma sekcjami

Możesz zainicjalizować Collab Chat w wielu oddzielnych sekcjach swojej strony. Każda sekcja będzie miała własne niezależne adnotacje:

[inline-code-attrs-start title = 'Collab Chat na wielu sekcjach'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    // Zainicjalizuj sekcję wstępu
    FastCommentsCollabChat(document.getElementById('intro-section'), {
        tenantId: 'demo',
        urlId: 'my-article-intro'
    });

    // Zainicjalizuj sekcję główną
    FastCommentsCollabChat(document.getElementById('main-section'), {
        tenantId: 'demo',
        urlId: 'my-article-main'
    });
</script>
[inline-code-end]

---