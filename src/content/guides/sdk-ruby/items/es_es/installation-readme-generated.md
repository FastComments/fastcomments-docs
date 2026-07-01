Add this line to your application's Gemfile:

```ruby
gem 'fastcomments'
```

And then execute:

```bash
bundle install
```

Or install it yourself as:

```bash
gem install fastcomments
```

### Library Contents

Esta biblioteca contiene el cliente API generado y las utilidades SSO para facilitar el trabajo con la API.

- [Documentación de la Biblioteca del Cliente API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Public vs Secured APIs

Para el cliente API, hay tres clases, `DefaultApi`, `PublicApi` y `ModerationApi`. La `DefaultApi` contiene métodos que requieren tu clave API, y la `PublicApi` contiene llamadas API que pueden realizarse directamente desde un navegador/dispositivo móvil, etc., sin autenticación. La `ModerationApi` contiene los métodos que impulsan el panel de moderador.

La `ModerationApi` ofrece una amplia suite de APIs de moderación en tiempo real y rápidas. Cada método de `ModerationApi` acepta un parámetro `sso` y puede autenticarse mediante SSO o una cookie de sesión de FastComments.com.