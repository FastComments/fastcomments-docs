### Executando Testes

```bash
# Defina as variáveis de ambiente
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# Execute os testes
pytest tests/
```

### Regenerando o Cliente

Para regenerar o cliente da API a partir da especificação OpenAPI mais recente:

```bash
./update.sh
```

Isso fará:
1. Baixar a especificação OpenAPI mais recente de um servidor FastComments em execução (ou usar openapi.yaml local)
2. Gerar o código cliente em Python
3. Achatar a estrutura de diretórios
4. Remover arquivos de configuração redundantes