Añade esta línea al Gemfile de tu aplicación:

```ruby
gem 'fastcomments'
```

Y luego ejecuta:

```bash
bundle install
```

O instálalo tú mismo con:

```bash
gem install fastcomments
```

### Contenido de la biblioteca

Esta biblioteca contiene el cliente de API generado y las utilidades SSO para facilitar el trabajo con la API.

- [Documentación de la biblioteca cliente de API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### APIs públicas vs APIs seguras

Para el cliente de API, hay tres clases: `DefaultApi`, `PublicApi` y `ModerationApi`. La `DefaultApi` contiene métodos que requieren tu clave de API, y la `PublicApi` contiene llamadas a la API
que pueden realizarse directamente desde un navegador/dispositivo móvil/etc sin autenticación. La `ModerationApi` contiene los métodos que alimentan el panel de moderación.

La `ModerationApi` ofrece un amplio conjunto de APIs de moderación en vivo y rápidas. Cada método de `ModerationApi` acepta un parámetro `sso` y puede autenticarse mediante SSO o una cookie de sesión de FastComments.com.