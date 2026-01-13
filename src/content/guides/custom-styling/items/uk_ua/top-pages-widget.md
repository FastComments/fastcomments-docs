Віджет Top Pages відображає список ваших сторінок із найбільшою кількістю коментарів.

Цей віджет містить мінімальне стилювання за замовчуванням і призначений для простого налаштування за допомогою власного CSS.

## Widget Structure

Віджет рендериться з наступною HTML-структурою:

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## Top Pages Default CSS Reference

Віджет включає наступне мінімальне стилювання за замовчуванням:

[inline-code-attrs-start title = 'CSS за замовчуванням віджета Top Pages'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## Customization Examples

### Додавання стилів до посилань
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

### Додавання меж між сторінками
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### Оформлення лічильника коментарів
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```

---