As páginas de novo webhook e de edição de webhook têm um botão `Send Test Payload` que envia uma requisição para a URL atualmente no formulário, tenha ela sido salva ou não. Os eventos Create e Update enviam um objeto WebhookComment fictício, enquanto o teste de Delete enviará um corpo de requisição fictício contendo apenas um ID.

## Verificando Payloads

Ao testar sua integração de webhook, verifique se as requisições recebidas incluem os seguintes cabeçalhos:

1. **`X-FastComments-Timestamp`** - Timestamp Unix (segundos)
2. **`X-FastComments-Signature`** - Assinatura HMAC-SHA256

Webhooks criados antes da introdução do esquema de assinatura também recebem um cabeçalho **`token`** contendo seu API Secret. Webhooks novos não recebem.

Use a verificação de assinatura HMAC para garantir que os payloads são autênticos.

## Ferramentas de Teste

Você pode usar ferramentas como [webhook.site](https://webhook.site) ou [ngrok](https://ngrok.com) para inspecionar os payloads de webhook recebidos durante o desenvolvimento.

## Tipos de Evento

- **Evento Create**: Disparado quando um novo comentário é criado.
- **Evento Update**: Disparado quando um comentário é editado.
- **Evento Delete**: Disparado quando um comentário é excluído.

Cada webhook está vinculado a um evento e a um método HTTP (POST, PUT ou DELETE). Cada evento inclui os dados completos do comentário no corpo da requisição (veja [Estruturas de Dados](/guide-webhooks.html#webhooks-structures) para o formato do payload).