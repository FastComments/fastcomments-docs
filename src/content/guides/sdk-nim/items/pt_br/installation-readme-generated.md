### Usando Nimble

```bash
nimble install fastcomments
```

### Compilando a partir do código-fonte

```bash
nimble build
```

### Conteúdo da Biblioteca

Esta biblioteca contém o cliente de API gerado e as utilidades SSO para facilitar o trabalho com a API.

- [Documentação da biblioteca do cliente da API](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### APIs Públicas vs APIs Seguras

Para o cliente de API, existem dois módulos de API, `api_default` e `api_public`. O `api_default` contém métodos que requerem sua chave de API, e o `api_public` contém chamadas de API que podem ser feitas diretamente de um navegador/dispositivo móvel/etc sem autenticação.