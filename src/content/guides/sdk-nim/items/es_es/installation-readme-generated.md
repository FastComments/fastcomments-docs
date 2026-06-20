---
### Uso de Nimble

```bash
nimble install fastcomments
```

### Compilando desde el código fuente

```bash
nimble build
```

### Contenido de la biblioteca

Esta biblioteca contiene el cliente de la API generado y las utilidades SSO para facilitar el trabajo con la API.

- [Documentación de la biblioteca del cliente API](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### APIs públicas vs protegidas

Para el cliente de la API, hay tres módulos de API, `api_default`, `api_public` y `api_moderation`. El `api_default` contiene métodos que requieren tu clave de API, y `api_public` contiene llamadas a la API
que se pueden realizar directamente desde un navegador/dispositivo móvil/etc. sin autenticación. El módulo `api_moderation` contiene métodos para el panel de moderación.

Los métodos de `api_moderation` abarcan el listado, el conteo, la búsqueda y la exportación de comentarios y sus registros; acciones de moderación como eliminar/restaurar comentarios, marcar, establecer el estado de revisión/spam/aprobación, ajustar votos y reabrir/cerrar hilos; prohibiciones (prohibir a un usuario de un comentario, deshacer una prohibición, resúmenes previos a la prohibición, estado y preferencias de la prohibición, y recuentos de usuarios prohibidos); e insignias y confianza (otorgar/quitar una insignia, listar insignias manuales, obtener/establecer el factor de confianza de un usuario, y obtener el perfil interno de un usuario). Cada método de `api_moderation` acepta un parámetro `sso` para que la llamada esté autenticada como moderador SSO.
---