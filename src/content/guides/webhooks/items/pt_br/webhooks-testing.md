Na administração de Webhooks existem botões `Send Test Payload` para cada tipo de evento (Create, Update, Delete). Os eventos Create e Update enviam um objeto fictício `WebhookComment`, enquanto o teste de Delete enviará um corpo de requisição fictício contendo apenas um ID.

## Verificando Payloads

Ao testar sua integração de webhook, verifique se as requisições recebidas incluem os seguintes cabeçalhos:

1. **`token`** - Your API Secret
2. **`X-FastComments-Timestamp`** - Unix timestamp (seconds)
3. **`X-FastComments-Signature`** - HMAC-SHA256 signature

Use a verificação de assinatura HMAC para garantir que os payloads sejam autênticos.

## Ferramentas de Teste

Você pode usar ferramentas como [webhook.site](https://webhook.site) ou [ngrok](https://ngrok.com) para inspecionar os payloads de webhook recebidos durante o desenvolvimento.

## Tipos de Evento

- **Create Event**: Acionado quando um novo comentário é criado. Método padrão: PUT
- **Update Event**: Acionado quando um comentário é editado. Método padrão: PUT
- **Delete Event**: Acionado quando um comentário é excluído. Método padrão: DELETE

Cada evento inclui todos os dados do comentário no corpo da requisição (veja [Estruturas de Dados](/guides/webhooks/webhooks-structures) para o formato do payload).