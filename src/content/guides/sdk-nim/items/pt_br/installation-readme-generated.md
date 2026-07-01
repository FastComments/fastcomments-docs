### Usando Nimble

```bash
nimble install fastcomments
```

### Compilando a partir do código-fonte

```bash
nimble build
```

### Conteúdo da Biblioteca

Esta biblioteca contém o cliente de API gerado e as utilitários SSO para facilitar o trabalho com a API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### APIs Públicas vs Seguras

Para o cliente de API, existem três módulos de API, `api_default`, `api_public` e `api_moderation`. O `api_default` contém métodos que requerem sua chave de API, e o `api_public` contém chamadas de API que podem ser feitas diretamente de um navegador/dispositivo móvel/etc sem autenticação. O módulo `api_moderation` contém métodos para o painel de moderador.

O módulo `api_moderation` fornece um conjunto extenso de APIs de moderação em tempo real e rápidas. Cada método `api_moderation` aceita um parâmetro `sso` e pode autenticar via SSO ou um cookie de sessão do FastComments.com.