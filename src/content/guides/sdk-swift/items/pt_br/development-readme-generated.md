---
### Regenerando o Cliente

Para regenerar o cliente da API a partir da especificação OpenAPI mais recente:

1. Certifique-se de que o servidor FastComments esteja em execução localmente em `http://localhost:3001`
2. Execute o script de atualização:

```bash
./update.sh
```

Isso fará:
- Baixar a especificação OpenAPI mais recente
- Gerar o código do cliente Swift (com documentação da API em client/docs)
- Compilar o pacote para verificar se tudo funciona
---