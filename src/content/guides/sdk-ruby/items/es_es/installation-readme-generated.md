Añade esta línea al Gemfile de tu aplicación:

```ruby
gem 'fastcomments'
```

Y luego ejecuta:

```bash
bundle install
```

O instálalo manualmente con:

```bash
gem install fastcomments
```

### Library Contents

Esta biblioteca contiene el cliente de API generado y las utilidades SSO para facilitar el trabajo con la API.

- [Documentación de la biblioteca del cliente de la API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Public vs Secured APIs

Para el cliente de la API, hay dos clases, `DefaultApi` y `PublicApi`. The `DefaultApi` contains methods that require your API key, and `PublicApi` contains api calls
that can be made directly from a browser/mobile device/etc without authentication.