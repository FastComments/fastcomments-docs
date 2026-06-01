La configuración proviene de tres lugares. Las fuentes posteriores prevalecen:

1. **Valores predeterminados globales** en `_config.yml` bajo la clave `fastcomments:`.
2. **Contexto de la página**, derivado automáticamente para los widgets con ámbito de página (véase más abajo).
3. **Atributos de la etiqueta** escritos en la propia etiqueta.

Por tanto, un `url_id` en la etiqueta anula el valor derivado de la página, que a su vez anula cualquier valor predeterminado global.

### Sintaxis de atributos

Los atributos son pares `key=value` en `snake_case`:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **Valores entrecomillados** (`"..."` o `'...'`) son cadenas literales.
- **No entrecomillados** `true`/`false` se convierten en booleanos, y los números se convierten en números.
- **No entrecomillados** cualquier otra cosa se resuelve como una variable de Liquid desde el contexto de la página, p. ej. `url_id=page.slug`. (Liquid no expande `{% raw %}\{{ ... }}{% endraw %}` dentro de los atributos de una etiqueta, así que use la forma sin adornos `page.slug` en lugar de `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Las claves de atributos y de configuración en snake_case se mapean automáticamente a las claves en camelCase que FastComments espera (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`, `has_dark_background` → `hasDarkBackground`, etc.). Cualquier otra opción de la [configuración del widget](https://docs.fastcomments.com/guide-customizations-and-configuration.html) pasa directamente de la misma manera.

### Valores derivados de la página

Para los widgets con ámbito de página (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`, `fastcomments_collab_chat`, `fastcomments_image_chat`) estos se completan automáticamente desde la página actual a menos que los establezca usted mismo:

- `url_id` ← `page.url` (un identificador estable independiente del dominio visitante)
- `url` ← `site.url` + `page.url` (solo cuando `url` está establecido en `_config.yml`)
- `page_title` ← `page.title`

Los widgets a nivel de sitio (comentarios/discusiones recientes, páginas principales, resumen de reseñas, feed de actividad de usuarios, recuento por lotes) no están ligados a una página y no derivan estos valores.

### Residencia de datos en la UE

Los clientes de la UE añaden `region: eu`, ya sea globalmente:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

o por etiqueta: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Los widgets entonces se cargan desde la CDN de la UE.