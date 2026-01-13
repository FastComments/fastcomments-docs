### Osnovni primer

Najpreprostejši način za uporabo Image Chat je ciljati en sam element slike. Ta primer prikazuje, kako omogočiti interaktivne razprave na sliki:

[inline-code-attrs-start title = 'Osnovni primer Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Primer z vsebnim elementom

Lahko tudi posredujete vsebnik, ki vsebuje sliko:

[inline-code-attrs-start title = 'Image Chat s vsebnim elementom'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Primer z lastnim URL ID

Privzeto Image Chat uporablja URL strani v kombinaciji z virom slike in koordinatami za identifikacijo pogovorov. Lahko določite lasten `urlId`:

[inline-code-attrs-start title = 'Image Chat z lastnim URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

To je uporabno, če se struktura vaših URL-jev spremeni, vendar želite obdržati iste pogovore, ali če želite deliti iste točke razprave na več straneh.

### Primer s temnim načinom

Če ima vaše spletno mesto temno ozadje in pripomoček tega ne zazna samodejno, lahko ročno omogočimo podporo za temni način:

[inline-code-attrs-start title = 'Image Chat s temnim načinom'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### Primer z nastavljeno velikostjo kvadratka

Velikost klikabilnih kvadratkov, ki se pojavijo na sliki, lahko prilagodite. Velikost je izražena kot odstotek širine slike:

[inline-code-attrs-start title = 'Image Chat z nastavljeno velikostjo kvadrata'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
            chatSquarePercentage: 2, // Manjši kvadratki (privzeto je 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### Primer s povratnim klicem za število komentarjev

Sledite, kdaj so komentarji dodani ali posodobljeni, z uporabo povratnega klica `commentCountUpdated`:

[inline-code-attrs-start title = 'Image Chat s povratnim klicem za število komentarjev'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### Primer z več slikami

Image Chat lahko inicializirate na več slikah. Vsaka slika bo imela svoje neodvisne točke razprave:

[inline-code-attrs-start title = 'Image Chat na več slikah'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // Inicializiraj na prvi sliki
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // Inicializiraj na drugi sliki
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---