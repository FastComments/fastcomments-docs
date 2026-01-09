Виджет «Top Pages» отображает список ваших страниц с наибольшим количеством комментариев.

Этот виджет содержит минимальные стили по умолчанию и разработан так, чтобы его было легко настроить с помощью ваших собственных CSS.

## Структура виджета

Виджет выводится со следующей HTML-структурой:

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## Справочник CSS по умолчанию для виджета «Top Pages»

Виджет содержит следующий минимальный набор стилей по умолчанию:

[inline-code-attrs-start title = 'CSS по умолчанию для виджета «Top Pages»'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## Примеры настройки

### Добавление стилей для ссылок
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

### Добавление границ между страницами
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### Стилизация счётчика комментариев
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```

---