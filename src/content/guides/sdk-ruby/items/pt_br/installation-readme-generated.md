Adicione esta linha ao Gemfile da sua aplicação:

```ruby
gem 'fastcomments'
```

E então execute:

```bash
bundle install
```

Ou instale você mesmo como:

```bash
gem install fastcomments
```

### Conteúdo da Biblioteca

Esta biblioteca contém o cliente de API gerado e os utilitários SSO para facilitar o trabalho com a API.

- [Documentação da Biblioteca do Cliente da API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### APIs Públicas vs APIs Seguras

Para o cliente de API, existem duas classes, `DefaultApi` e `PublicApi`. O `DefaultApi` contém métodos que exigem sua chave de API, e `PublicApi` contém chamadas de API que podem ser feitas diretamente a partir de um navegador/dispositivo móvel/etc sem autenticação.