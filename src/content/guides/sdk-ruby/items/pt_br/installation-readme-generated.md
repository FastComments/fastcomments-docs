Adicione esta linha ao Gemfile da sua aplicação:

```ruby
gem 'fastcomments'
```

Em seguida, execute:

```bash
bundle install
```

Ou instale você mesmo como:

```bash
gem install fastcomments
```

### Conteúdo da Biblioteca

Esta biblioteca contém o cliente de API gerado e as utilidades SSO para facilitar o trabalho com a API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### APIs Públicas vs Seguras

Para o cliente de API, existem três classes, `DefaultApi`, `PublicApi` e `ModerationApi`. O `DefaultApi` contém métodos que requerem sua chave de API, e o `PublicApi` contém chamadas de API  
que podem ser feitas diretamente de um navegador/dispositivo móvel/etc sem autenticação. O `ModerationApi` contém os métodos que alimentam o painel de moderador.

O `ModerationApi` fornece um conjunto extenso de APIs de moderação ao vivo e rápidas. Cada método do `ModerationApi` aceita um parâmetro `sso` e pode autenticar via SSO ou um cookie de sessão do FastComments.com.