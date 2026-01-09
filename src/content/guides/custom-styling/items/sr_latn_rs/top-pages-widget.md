Vidžet Najkomentisanije stranice prikazuje listu vaših stranica sa najviše komentara.

Ovaj vidžet sadrži minimalni podrazumevani stil i dizajniran je tako da ga lako možete prilagoditi sopstvenim CSS-om.

## Struktura vidžeta

Vidžet se prikazuje sa sledećom HTML strukturom:

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## Referenca podrazumevanog CSS-a za Najkomentisanije stranice

Vidžet uključuje sledeći minimalni podrazumevani stil:

[inline-code-attrs-start title = 'Podrazumevani CSS vidžeta Najkomentisanije stranice'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## Primeri prilagođavanja

### Dodavanje stilova za linkove
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

### Dodavanje ivica između stranica
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### Stilizovanje broja komentara
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```

---