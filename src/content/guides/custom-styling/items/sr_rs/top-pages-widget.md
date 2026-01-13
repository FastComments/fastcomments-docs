The Top Pages widget displays a list of your most commented pages.

Овај видгет приказује листу ваших највише коментарисаних страница.

This widget includes minimal default styling and is designed to be easily customized with your own CSS.

Овај видгет садржи минималне подразумеване стилове и дизајниран је да буде лак за прилагођавање вашим CSS-ом.

## Widget Structure

## Структура видгета

The widget renders with the following HTML structure:

Видгет се рендерује са следећом HTML структуром:

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## Top Pages Default CSS Reference

## Подразумевани CSS видгета Највише коментарисаних страница

The widget includes the following minimal default styling:

Видгет укључује следећи минимални подразумевани стил:

[inline-code-attrs-start title = 'Подразумевани CSS видгета Највише коментарисаних страница'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## Customization Examples

## Примери прилагођавања

### Add Styling to Links

### Додавање стила линковима
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

### Add Borders Between Pages

### Додавање ивица између страница
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### Style the Comment Count

### Обликовање броја коментара
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```