Top Pages widget'ı en çok yorum alan sayfalarınızın bir listesini gösterir.

Bu widget, minimal varsayılan stil içerir ve kendi CSS'inizle kolayca özelleştirilecek şekilde tasarlanmıştır.

## Widget Yapısı

Widget aşağıdaki HTML yapısıyla render edilir:

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## En Çok Yorum Alan Sayfalar Varsayılan CSS Referansı

Widget aşağıdaki minimal varsayılan stilleri içerir:

[inline-code-attrs-start title = 'En Çok Yorum Alan Sayfalar Widget Varsayılan CSS'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## Özelleştirme Örnekleri

### Bağlantılara Stil Ekle
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

### Sayfalar Arasına Kenarlık Ekle
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### Yorum Sayısını Stilize Et
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```

---