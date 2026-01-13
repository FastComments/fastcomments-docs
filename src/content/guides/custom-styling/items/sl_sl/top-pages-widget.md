Pripomoček Najbolj komentirane strani prikazuje seznam vaših strani z največ komentarji.

Ta pripomoček vključuje minimalno privzeto oblikovanje in je zasnovan tako, da ga lahko zlahka prilagodite s svojim CSS-jem.

## Struktura pripomočka

Pripomoček se izriše z naslednjo HTML strukturo:

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## Privzeta CSS referenca pripomočka Najbolj komentirane strani

[inline-code-attrs-start title = 'Privzeta CSS pripomočka Najbolj komentirane strani'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## Primeri prilagoditev

### Dodaj slog povezavam
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

### Dodaj obrobe med stranmi
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### Stiliziraj število komentarjev
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```

---