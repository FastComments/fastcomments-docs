### Podstawowy przykład

Najprostszy sposób użycia Image Chat to skierowanie na pojedynczy element obrazu. Ten przykład pokazuje, jak włączyć interaktywne dyskusje na obrazie:

[inline-code-attrs-start title = 'Podstawowy przykład Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Product Image with Chat</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

### Przykład z elementem kontenera

Możesz też przekazać element kontenera, który zawiera wewnątrz obraz:

[inline-code-attrs-start title = 'Image Chat z kontenerem'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<div id="image-container">
    <img src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="System Diagram" />
</div>

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('image-container'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

### Przykład z niestandardowym identyfikatorem URL

Domyślnie Image Chat używa adresu URL strony połączonego ze źródłem obrazu i współrzędnymi do identyfikacji konwersacji. Możesz podać niestandardowy `urlId`:

[inline-code-attrs-start title = 'Image Chat z niestandardowym identyfikatorem URL'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

Jest to przydatne, jeśli struktura adresów URL ulegnie zmianie, a chcesz zachować te same konwersacje, lub jeśli chcesz udostępniać te same punkty dyskusji na wielu stronach.

### Przykład z trybem ciemnym

Jeżeli Twoja strona ma ciemne tło i widżet nie wykrywa go automatycznie tak jak powinien, możemy ręcznie włączyć obsługę trybu ciemnego:

[inline-code-attrs-start title = 'Image Chat z trybem ciemnym'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### Przykład z niestandardowym rozmiarem kwadratu czatu

Możesz dostosować rozmiar klikalnych kwadratów wyświetlanych na obrazie. Rozmiar określany jest jako procent szerokości obrazu:

[inline-code-attrs-start title = 'Image Chat z niestandardowym rozmiarem kwadratu'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Image Chat with Custom Square Size</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo',
            chatSquarePercentage: 2, // Mniejsze kwadraty (domyślnie 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### Przykład z funkcją zwrotną aktualizacji liczby komentarzy

Śledź, kiedy komentarze są dodawane lub aktualizowane, używając funkcji zwrotnej `commentCountUpdated`:

[inline-code-attrs-start title = 'Image Chat z funkcją zwrotną aktualizacji liczby komentarzy'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        commentCountUpdated: function(count) {
            console.log('Total comments:', count);
            document.getElementById('comment-badge').textContent = count;
        }
    });
</script>
[inline-code-end]

### Przykład z wieloma obrazami

Możesz zainicjalizować Image Chat na wielu obrazach. Każdy obraz będzie miał swoje własne, niezależne punkty dyskusji:

[inline-code-attrs-start title = 'Image Chat na wielu obrazach'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // Zainicjalizuj na pierwszym obrazie
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // Zainicjalizuj na drugim obrazie
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---