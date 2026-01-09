### Osnovni primer

Najjednostavniji način korišćenja Image Chat-a je ciljanje jednog elementa slike. Ovaj primer pokazuje kako omogućiti interaktivne diskusije na slici:

[inline-code-attrs-start title = 'Osnovni primer Image Chata'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Primer sa elementom kontejnera

Takođe možete proslediti element kontejnera koji sadrži sliku:

[inline-code-attrs-start title = 'Image Chat sa kontejnerom'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Primer sa prilagođenim URL ID-jem

Po defaultu, Image Chat koristi URL stranice kombinovan sa izvorom slike i koordinatama kako bi identifikovao razgovore. Možete navesti prilagođeni `urlId`:

[inline-code-attrs-start title = 'Image Chat sa prilagođenim URL ID-jem'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

Ovo je korisno ako se struktura vaših URL-ova promeni, ali želite da zadržite iste razgovore, ili ako želite da delite iste tačke diskusije na više stranica.

### Primer sa tamnim režimom

Ako vaša stranica ima tamnu pozadinu i vidžet je ne detektuje automatski kako bi trebalo, možemo ručno omogućiti podršku za tamni režim:

[inline-code-attrs-start title = 'Image Chat sa tamnim režimom'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### Primer sa prilagođenom veličinom kvadratića za chat

Možete podesiti veličinu klikabilnih kvadratića koji se pojavljuju na slici. Veličina se navodi kao procenat širine slike:

[inline-code-attrs-start title = 'Image Chat sa prilagođenom veličinom kvadrata'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
            chatSquarePercentage: 2, // Manji kvadratići (podrazumevano je 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### Primer sa callback-om za broj komentara

Pratite kada se komentari dodaju ili ažuriraju koristeći callback `commentCountUpdated`:

[inline-code-attrs-start title = 'Image Chat sa callback-om za broj komentara'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### Primer sa više slika

Možete inicijalizovati Image Chat na više slika. Svaka slika će imati svoje nezavisne tačke diskusije:

[inline-code-attrs-start title = 'Image Chat na više slika'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // Inicijalizuj na prvoj slici
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // Inicijalizuj na drugoj slici
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---