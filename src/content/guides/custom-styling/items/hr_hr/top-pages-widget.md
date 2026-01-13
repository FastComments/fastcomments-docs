The Top Pages widget prikazuje popis vaših stranica s najviše komentara.

Ovaj widget sadrži minimalne zadane stilove i dizajniran je da se jednostavno prilagodi vlastitim CSS-om.

## Widget Structure

Widget se prikazuje sa sljedećom HTML strukturom:

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## Top Pages Default CSS Reference

Widget uključuje sljedeće minimalne zadane stilove:

[inline-code-attrs-start title = 'Zadani CSS widgeta Top Pages'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## Customization Examples

### Dodavanje stilova za poveznice
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

### Dodavanje obruba između stranica
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### Stiliziranje broja komentara
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```