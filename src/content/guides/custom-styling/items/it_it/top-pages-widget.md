Il widget Pagine principali mostra un elenco delle tue pagine con più commenti.

Questo widget include uno stile minimale predefinito ed è progettato per essere facilmente personalizzato con il tuo CSS.

## Struttura del widget

Il widget viene renderizzato con la seguente struttura HTML:

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## Riferimento CSS predefinito del widget Pagine principali

Il widget include il seguente stile minimale predefinito:

[inline-code-attrs-start title = 'CSS predefinito del widget Pagine principali'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## Esempi di personalizzazione

### Aggiungere stile ai link
```css
.fastcomments-top-pages .title-link {
    color: #0066cc !important;
    text-decoration: none !important;
    font-size: 14px !important;
}

.fastcomments-top-pages .title-link:hover {
    text-decoration: underline !important;
}
```

### Aggiungere bordi tra le pagine
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### Stilizzare il conteggio dei commenti
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```

---