---
### 401 Erros Não Autorizados

Se você está recebendo erros 401 ao usar a API autenticada:

1. **Verifique sua chave de API**: Certifique-se de que você está usando a chave de API correta do seu painel do FastComments
2. **Verifique o tenant ID**: Certifique-se de que o tenant ID corresponde à sua conta
3. **Formato da chave de API**: A chave de API deve ser definida no cliente da API:

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **Usando a API errada**: Certifique-se de que está usando `DefaultAPI` (não `PublicAPI`) para chamadas autenticadas

### Problemas com tokens SSO

Se os tokens SSO não estiverem funcionando:

1. **Use o modo seguro em produção**: Sempre use `FastCommentsSSO.createSecure()` com sua chave de API em produção
2. **Somente no servidor**: Gere tokens SSO seguros no seu servidor, nunca exponha sua chave de API para clientes
3. **Verifique os dados do usuário**: Garanta que todos os campos obrigatórios (id, email, username) sejam fornecidos
4. **Expiração do token**: Tokens SSO seguros incluem um carimbo de data/hora e podem expirar. Gere tokens novos conforme necessário.

### Erros de SSL/TLS

Se você encontrar erros de SSL/TLS:

1. Certifique-se de que o Info.plist do seu app permita conexões HTTPS para fastcomments.com
2. Verifique se você não está usando exceções do App Transport Security que possam bloquear a conexão
---