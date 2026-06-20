Adicione esta linha ao Gemfile da sua aplicação:

```ruby
gem 'fastcomments'
```

Em seguida, execute:

```bash
bundle install
```

Ou instale-o manualmente com:

```bash
gem install fastcomments
```

### Conteúdo da Biblioteca

Esta biblioteca contém o cliente de API gerado e as utilidades SSO para facilitar o trabalho com a API.

- [Documentação da biblioteca do cliente da API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### APIs Públicas vs Seguras

Para o cliente de API, existem três classes, `DefaultApi`, `PublicApi`, e `ModerationApi`. O `DefaultApi` contém métodos que exigem sua chave de API, e o `PublicApi` contém chamadas da API que podem ser feitas diretamente de um navegador/dispositivo móvel/etc. sem autenticação. O `ModerationApi` contém os métodos que dão suporte ao painel de moderador.

O `ModerationApi` abrange moderação de comentários (listar, contar, buscar, registros, exportar), ações de moderação (remover/restaurar, denunciar, definir status de revisão/spam/aprovação, votos, reabrir/fechar tópico), banimentos (banir a partir de um comentário, desfazer, resumos pré-banimento, status/preferências de ban, contagens de usuários banidos), e distintivos & confiança (conceder/remover distintivo, distintivos manuais, obter/definir fator de confiança, perfil interno do usuário). Cada método do `ModerationApi` aceita um parâmetro `sso` para que a requisição possa ser feita em nome de um moderador autenticado via SSO.