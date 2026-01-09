Le widget Top Pages affiche une liste de vos pages les plus commentées.

Ce widget inclut un style par défaut minimal et est conçu pour être facilement personnalisé avec votre propre CSS.

## Structure du widget

Le widget s'affiche avec la structure HTML suivante :

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## Référence CSS par défaut de Top Pages

Le widget inclut le style par défaut minimal suivant :

[inline-code-attrs-start title = 'CSS par défaut du widget Top Pages'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## Exemples de personnalisation

### Ajouter du style aux liens
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

### Ajouter des bordures entre les pages
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### Styliser le nombre de commentaires
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```