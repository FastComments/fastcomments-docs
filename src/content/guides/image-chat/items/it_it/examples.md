### Esempio base

Il modo più semplice per utilizzare Image Chat è puntare a un singolo elemento immagine. Questo esempio mostra come abilitare discussioni interattive su un'immagine:

[inline-code-attrs-start title = 'Esempio base di Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Esempio con elemento contenitore

Puoi anche passare un elemento contenitore che al suo interno contiene un'immagine:

[inline-code-attrs-start title = 'Image Chat con contenitore'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Esempio con URL ID personalizzato

Per impostazione predefinita, Image Chat utilizza l'URL della pagina combinato con la sorgente dell'immagine e le coordinate per identificare le conversazioni. Puoi fornire un `urlId` personalizzato:

[inline-code-attrs-start title = 'Image Chat con URL ID personalizzato'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

Questo è utile se la struttura degli URL cambia ma vuoi mantenere le stesse conversazioni, oppure se vuoi condividere gli stessi punti di discussione su più pagine.

### Esempio con modalità scura

Se il tuo sito ha uno sfondo scuro e il widget non lo rileva automaticamente come dovrebbe, possiamo abilitare manualmente il supporto per la modalità scura:

[inline-code-attrs-start title = 'Image Chat con modalità scura'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### Esempio con dimensione personalizzata dei quadrati di chat

Puoi regolare la dimensione dei quadrati cliccabili che appaiono sull'immagine. La dimensione è specificata come percentuale della larghezza dell'immagine:

[inline-code-attrs-start title = 'Image Chat con dimensione quadrato personalizzata'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
            chatSquarePercentage: 2, // Quadrati più piccoli (predefinito: 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### Esempio con callback per il conteggio dei commenti

Monitora quando i commenti vengono aggiunti o aggiornati utilizzando la callback `commentCountUpdated`:

[inline-code-attrs-start title = 'Image Chat con callback per conteggio commenti'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### Esempio con immagini multiple

Puoi inizializzare Image Chat su più immagini. Ogni immagine avrà i propri punti di discussione indipendenti:

[inline-code-attrs-start title = 'Image Chat su più immagini'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // Inizializza sulla prima immagine
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // Inizializza sulla seconda immagine
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---