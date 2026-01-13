Das Top Pages-Widget zeigt eine Liste Ihrer am meisten kommentierten Seiten an.

Dieses Widget enthält minimale Standardstile und ist so konzipiert, dass es leicht mit Ihrem eigenen CSS angepasst werden kann.

## Widget-Struktur

Das Widget rendert die folgende HTML-Struktur:

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## Standard-CSS-Referenz des Top Pages-Widgets

Das Widget enthält die folgende minimale Standardstilisierung:

[inline-code-attrs-start title = 'Standard-CSS des Top Pages-Widgets'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## Anpassungsbeispiele

### Styling für Links hinzufügen
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

### Rahmen zwischen Seiten hinzufügen
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### Kommentaranzahl gestalten
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```