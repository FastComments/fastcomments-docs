No painel de administração de Webhooks existem botões `Send Test Payload` para cada tipo de evento (Create, Update, Delete). Os eventos Create e Update enviam um objeto dummy `WebhookComment`, enquanto testar Delete enviará um corpo de requisição dummy com apenas um ID.

## Verificando Payloads

Ao testar sua integração de webhook, verifique se as requisições recebidas incluem os seguintes cabeçalhos:

1. **`token`** - Seu Segredo da API
2. **`X-FastComments-Timestamp`** - Marca de tempo Unix (segundos)
3. **`X-FastComments-Signature`** - assinatura HMAC-SHA256

Use a verificação da assinatura HMAC para garantir que os payloads são autênticos.

## Ferramentas de Teste

Você pode usar ferramentas como [webhook.site](https://webhook.site) ou [ngrok](https://ngrok.com) para inspecionar os payloads de webhook recebidos durante o desenvolvimento.

## Tipos de Eventos

- **Create Event**: Disparado quando um novo comentário é criado. Método padrão: PUT
- **Update Event**: Disparado quando um comentário é editado. Método padrão: PUT
- **Delete Event**: Disparado quando um comentário é excluído. Método padrão: DELETE

Cada evento inclui os dados completos do comentário no corpo da requisição (veja [Estruturas de Dados](/guide-webhooks.html#webhooks-structures) para o formato do payload).