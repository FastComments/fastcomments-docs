### Osnovni primjer

Najjednostavniji način korištenja Image Chata je ciljanje jednog elementa slike. Ovaj primjer pokazuje kako omogućiti interaktivne rasprave na slici:

[inline-code-attrs-start title = 'Osnovni primjer Image Chata'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Primjer s kontejnerskim elementom

Također možete proslijediti kontejnerski element koji sadrži sliku:

[inline-code-attrs-start title = 'Image Chat s kontejnerom'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Primjer s prilagođenim URL ID-om

Po defaultu, Image Chat koristi URL stranice u kombinaciji s izvorom slike i koordinatama za identifikaciju razgovora. Možete navesti prilagođeni `urlId`:

[inline-code-attrs-start title = 'Image Chat s prilagođenim URL ID-om'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

Ovo je korisno ako se struktura vaših URL-ova promijeni, ali želite zadržati iste razgovore, ili ako želite dijeliti iste točke rasprave na više stranica.

### Primjer s tamnim načinom rada

Ako vaša stranica ima tamnu pozadinu i widget je ne detektira automatski kako treba, možemo ručno omogućiti podršku za tamni način rada:

[inline-code-attrs-start title = 'Image Chat s tamnim načinom rada'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### Primjer s prilagođenom veličinom kvadrata chata

Možete prilagoditi veličinu klikabilnih kvadrata koji se pojavljuju na slici. Veličina je specificirana kao postotak širine slike:

[inline-code-attrs-start title = 'Image Chat s prilagođenom veličinom kvadrata'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
            chatSquarePercentage: 2, // Manji kvadrati (zadano je 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### Primjer s povratnim pozivom broja komentara

Pratite kada su komentari dodani ili ažurirani koristeći `commentCountUpdated` povratni poziv:

[inline-code-attrs-start title = 'Image Chat s povratnim pozivom za broj komentara'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### Primjer s više slika

Možete inicijalizirati Image Chat na više slika. Svaka slika će imati svoje neovisne točke rasprave:

[inline-code-attrs-start title = 'Image Chat na više slika'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // Inicijaliziraj na prvoj slici
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // Inicijaliziraj na drugoj slici
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---