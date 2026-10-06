---
Usuários marcados receberão um e‑mail informando que foram marcados ou mencionados em um comentário.

[app-screenshot-start url='/test-e2e/email/comment-user-mention?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"Hey%20%40winrid%20I%20wanted%20you%20to%20see%20this."%2C"commentHTML"%3A"Hey%20<b>%40winrid<%2Fb>%20I%20wanted%20you%20to%20see%20this."%2C"date"%3A1633998787864%2C"pageTitle"%3A"Some%20Page%20Title"%7D&username=winrid&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&INTRO=Hey%20winrid%2C&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&unsubscribeLink=%7B"url"%3A"%2Fauth%2Fmy-account%2Fedit-notifications"%2C"textId"%3A"UNSUBSCRIBE_HERE"%7D&viewCommentUrl=https%3A%2F%2Fexample.com%23fast-comments-jt%3Dsome-db-id&locale=en_us&canReplyByEmail=true&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Corpo do e‑mail de notificação citando um comentário com a menção @winrid em negrito, mais links de visualização e cancelamento de inscrição'; title='Notificação de Usuário Mencionado' app-screenshot-end]

Desativar as notificações impedirá esses e‑mails, e um cabeçalho é incluído em cada e‑mail para que os clientes de e‑mail permitam que o usuário cancele a inscrição de forma simples.

Usuários marcados também recebem uma notificação no aplicativo, mesmo quando não possuem endereço de e‑mail cadastrado.

Se o comentário estiver aguardando aprovação do moderador, o e‑mail de menção é retido e enviado assim que o comentário for aprovado. Ele não é enviado se o comentário for marcado como spam, excluído ou não aprovado dentro de 7 dias.

Para ver se um usuário mencionado recebeu e‑mail, ou por que não recebeu, abra os [Logs de Comentário](/guide-moderation.html#comment-logs) do comentário na página Moderar Comentários.

---