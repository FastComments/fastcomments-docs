Agrega esta línea al Gemfile de tu aplicación:

```ruby
gem 'fastcomments'
```

Y luego ejecuta:

```bash
bundle install
```

O instálalo tú mismo como:

```bash
gem install fastcomments
```

### Contenido de la biblioteca

Esta biblioteca contiene el cliente de API generado y las utilidades SSO para facilitar el trabajo con la API.

- [Documentación de la biblioteca del cliente de API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### APIs públicas vs protegidas

Para el cliente de API, hay tres clases, `DefaultApi`, `PublicApi`, y `ModerationApi`. La `DefaultApi` contiene métodos que requieren tu clave de API, y `PublicApi` contiene llamadas de API
que pueden hacerse directamente desde un navegador/dispositivo móvil/etc. sin autenticación. La `ModerationApi` contiene los métodos que impulsan el panel de moderación.

La `ModerationApi` cubre la moderación de comentarios (listar, contar, buscar, registros, exportar), acciones de moderación (eliminar/restaurar, marcar, establecer estado de revisión/spam/aprobación, votos, reabrir/cerrar hilo),
prohibiciones (banear a un usuario desde un comentario, deshacer, resúmenes previos al baneo, estado/preferencias de baneo, recuentos de usuarios baneados), e insignias y confianza (otorgar/quitar insignia, insignias manuales, obtener/establecer factor de confianza, perfil interno del usuario).
Cada método de `ModerationApi` acepta un parámetro `sso` para que la solicitud pueda hacerse en nombre de un moderador autenticado por SSO.