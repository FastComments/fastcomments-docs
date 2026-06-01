Todas las opciones del widget FastComments se configuran bajo `[params.fastcomments]` en `hugo.toml`, y pueden sobrescribirse por página en el front matter bajo `[fastcomments]`. Precedencia, de menor a mayor: site params, front matter de la página, parámetros del shortcode.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# front matter de una página
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

Cuando no se proporciona ni `url` ni `urlId`, `url` por defecto es el permalink de la página para que los hilos de comentarios permanezcan vinculados a una URL estable.

### Residencia de datos en la UE

Los clientes de la UE establecen `region = "eu"` para enrutar el widget a `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Nota sobre el uso de mayúsculas en las claves

Hugo convierte a minúsculas cada clave en `hugo.toml` y el front matter, pero los widgets de FastComments requieren claves en camelCase (`tenantId`, `hasDarkBackground`). Este componente restaura automáticamente la capitalización correcta para cada opción conocida de primer nivel, así que escriba las opciones en su forma normal camelCase. Las claves anidadas dentro de un valor objeto (por ejemplo las claves de un mapa `translations`, o los campos de `pageReactConfig`) no se restauran. Configure esas a través de la interfaz de personalización del panel de FastComments en su lugar.