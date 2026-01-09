Top Pages видџет приказује списак ваших страница са највише коментара.

Овај видџет укључује минимални подразумевани стил и дизајниран је тако да се лако може прилагодити вашим CSS-ом.

## Структура видџета

Видџет се рендерује са следећом HTML структуром:

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## Подразумевани CSS за Top Pages

Видџет укључује следећи минимални подразумевани стил:

[inline-code-attrs-start title = 'Подразумевани CSS за Top Pages видџет'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## Примери прилагођавања

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

### Стилизација броја коментара
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```

---