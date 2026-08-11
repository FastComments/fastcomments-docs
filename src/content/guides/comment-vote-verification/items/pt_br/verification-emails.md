When a user leaves a comment, or vote, and they are not logged in, or their account is  
não verificada, they will receive an email asking them to verify this action.

However, we do our best not to spam your users with emails, and won't send more than one  
e‑mail de verificação por sessão. See the Sessions section for more details.

By default, the comment verification emails look like the following:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Corpo padrão de e‑mail de verificação citando o comentário de Alexander com um botão para confirmar a publicação'; title='E‑mail de Verificação de Comentário' app-screenshot-end]

By default, the vote verification emails look like the following:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='E‑mail padrão pedindo a Devon que confirme um voto, mostrando o comentário que foi votado e um botão de confirmação'; title='E‑mail de Verificação de Voto' app-screenshot-end]

By default, FastComments will show its logo and name in the footer of these emails:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='Base de um e‑mail de verificação mostrando o logotipo e nome padrão da FastComments no rodapé'; title='Rodapé do E‑mail' app-screenshot-end]

If you are on the Flex or Pro tiers, [O nome do remetente, e‑mail e a identidade visual podem ser personalizados](/guide-multiple-sites.html#from-name-email-logo).