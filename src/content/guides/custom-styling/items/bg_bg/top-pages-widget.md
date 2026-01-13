Уиджетът Top Pages показва списък с най-коментираните ви страници.

Този уиджет включва минимално подразбиращо се стилизиране и е проектиран да бъде лесно персонализиран с вашия собствен CSS.

## Структура на уиджета

Уиджетът се рендира със следната HTML структура:

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## Подразбиращ се CSS за Top Pages

Уиджетът включва следното минимално подразбиращо се стилизиране:

[inline-code-attrs-start title = 'Подразбиращ се CSS за уиджета Top Pages'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## Примери за персонализиране

### Добавяне на стилове към връзките
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

### Добавяне на разделителни линии между страниците
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### Стилиране на броя коментари
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```

---