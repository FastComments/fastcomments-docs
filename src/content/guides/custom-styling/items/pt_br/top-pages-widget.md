O widget Top Pages exibe uma lista das suas páginas mais comentadas.

Este widget inclui um estilo padrão mínimo e foi projetado para ser facilmente personalizado com seu próprio CSS.

## Estrutura do Widget

O widget é renderizado com a seguinte estrutura HTML:

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## Referência de CSS Padrão do widget Top Pages

O widget inclui o seguinte estilo padrão mínimo:

[inline-code-attrs-start title = 'CSS padrão do widget Top Pages'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## Exemplos de Personalização

### Adicionar Estilo aos Links
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

### Adicionar Bordas Entre as Páginas
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### Estilizar a Contagem de Comentários
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```

---