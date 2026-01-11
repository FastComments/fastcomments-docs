No painel de administração de Webhooks há botões `Send Test Payload` para cada tipo de evento (Create, Update, Delete). Os eventos Create e Update enviam um objeto WebhookComment de teste, enquanto o teste de Delete enviará um corpo de requisição de teste contendo apenas um ID.

O teste fará duas chamadas para verificar o código de resposta para os cenários "happy" (API Key correto) e "sad" (API Key inválido).

Quando o teste envia uma API Key inválida, você deve retornar um código de status 401 para que o teste seja considerado totalmente aprovado. Se você não verificar corretamente o valor do token, verá um erro.

Isso serve para garantir que você autentique corretamente a requisição.