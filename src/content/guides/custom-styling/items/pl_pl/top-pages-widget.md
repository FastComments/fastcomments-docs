Widżet Najpopularniejsze strony wyświetla listę twoich najbardziej komentowanych stron.

Ten widżet zawiera minimalne domyślne style i został zaprojektowany tak, aby można go było łatwo dostosować za pomocą własnego CSS.

## Struktura widżetu

Widżet renderuje się przy następującej strukturze HTML:

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## Domyślne style CSS widżetu Najpopularniejsze strony

Widżet zawiera następujące minimalne domyślne style:

[inline-code-attrs-start title = 'Domyślne style CSS widżetu Najpopularniejsze strony'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## Przykłady dostosowania

### Dodawanie stylów do linków
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

### Dodawanie obramowań między stronami
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### Stylowanie liczby komentarzy
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```

---