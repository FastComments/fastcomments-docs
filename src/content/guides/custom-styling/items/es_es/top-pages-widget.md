El widget Páginas principales muestra una lista de tus páginas con más comentarios.

Este widget incluye un estilo predeterminado mínimo y está diseñado para ser fácilmente personalizado con tu propio CSS.

## Estructura del widget

El widget se renderiza con la siguiente estructura HTML:

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## Referencia del CSS predeterminado de Páginas principales

El widget incluye el siguiente estilo predeterminado mínimo:

[inline-code-attrs-start title = 'CSS predeterminado del widget Páginas principales'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## Ejemplos de personalización

### Agregar estilo a los enlaces
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

### Agregar bordes entre páginas
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### Estilizar el número de comentarios
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```