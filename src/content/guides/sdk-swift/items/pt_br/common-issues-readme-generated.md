### Erros 401 Não Autorizado

Se você estiver recebendo erros 401 ao usar a API autenticada:

1. **Verifique sua chave de API**: Certifique‑se de que está usando a chave de API correta do seu painel do FastComments
2. **Verifique o ID do tenant**: Certifique‑se de que o ID do tenant corresponde à sua conta
3. **Formato da chave de API**: A chave de API deve ser definida como o cabeçalho `x-api-key` na configuração compartilhada:

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **Usando a API errada**: Certifique‑se de que está usando `DefaultAPI` (não `PublicAPI`) para chamadas autenticadas

### Problemas com Token SSO

Se os tokens SSO não estiverem funcionando:

1. **Use modo seguro em produção**: Sempre use `FastCommentsSSO.createSecure()` com sua chave de API para produção
2. **Somente no lado do servidor**: Gere tokens SSO seguros no seu servidor, nunca exponha sua chave de API aos clientes
3. **Verifique os dados do usuário**: Certifique‑se de que todos os campos obrigatórios (id, email, nome de usuário) foram fornecidos
4. **Expiração do token**: Tokens SSO seguros incluem um carimbo de tempo e podem expirar. Gere novos tokens conforme necessário.

### Erros SSL/TLS

Se você encontrar erros SSL/TLS:

1. Certifique‑se de que o Info.plist do seu app permite conexões HTTPS para fastcomments.com
2. Verifique se você não está usando exceções do App Transport Security que possam bloquear a conexão.