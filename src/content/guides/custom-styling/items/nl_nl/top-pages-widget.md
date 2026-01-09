De Top Pages-widget toont een lijst van uw meest becommentarieerde pagina's.

Deze widget bevat minimale standaardstyling en is ontworpen om eenvoudig met uw eigen CSS aangepast te worden.

## Widgetstructuur

De widget wordt weergegeven met de volgende HTML-structuur:

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## Standaard CSS-referentie voor Top Pages

De widget bevat de volgende minimale standaardstyling:

[inline-code-attrs-start title = 'Standaard CSS van Top Pages-widget'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## Aanpassingsvoorbeelden

### Stijl toevoegen aan links
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

### Randen toevoegen tussen pagina's
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### Stijl het aantal reacties
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```