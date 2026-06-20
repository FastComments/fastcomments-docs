### Usando Nimble

```bash
nimble install fastcomments
```

### Compilando a partir do Código-Fonte

```bash
nimble build
```

### Conteúdo da Biblioteca

Esta biblioteca contém o cliente de API gerado e as utilidades SSO para facilitar o trabalho com a API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### APIs Públicas vs Seguras

Para o cliente de API, existem três módulos de API, `api_default`, `api_public` e `api_moderation`. O `api_default` contém métodos que exigem sua chave de API, e o `api_public` contém chamadas de API
que podem ser feitas diretamente de um navegador/dispositivo móvel/etc sem autenticação. O módulo `api_moderation` contém métodos para o painel do moderador.

Os métodos de `api_moderation` abrangem listagem, contagem, busca e exportação de comentários e seus registros; ações de moderação como remover/restaurar comentários, sinalizar, definir status de revisão/spam/aprovação, ajustar votos e reabrir/fechar tópicos; banimentos (banir um usuário de um comentário, desfazer um banimento, resumos pré-banimento, status e preferências de banimento e contagens de usuários banidos); e insígnias e confiança (conceder/remover uma insígnia, listar insígnias manuais, obter/definir o fator de confiança de um usuário e recuperar o perfil interno de um usuário). Cada método de `api_moderation` aceita um parâmetro `sso` para que a chamada seja autenticada como um moderador SSO.