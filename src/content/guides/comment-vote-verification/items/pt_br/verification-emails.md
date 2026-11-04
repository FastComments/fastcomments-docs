Quando um usuário deixa um comentário, ou um voto, e não está autenticado, ou sua conta não está
verificada, ele receberá um e-mail pedindo para verificar essa ação.

No entanto, fazemos o nosso melhor para não enviar spam aos seus usuários com e-mails, e não enviaremos mais de um
e-mail de verificação por sessão. Consulte a seção Sessions para mais detalhes.

Por padrão, os e-mails de verificação de comentários têm a seguinte aparência:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Corpo padrão do e-mail de verificação citando o comentário de Alexander com um botão para confirmar a publicação'; title='E-mail de Verificação de Comentário' app-screenshot-end]

Por padrão, os e-mails de verificação de votos têm a seguinte aparência:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='E-mail padrão pedindo que Devon confirme um voto, mostrando o comentário que recebeu o voto e um botão de confirmação'; title='E-mail de Verificação de Voto' app-screenshot-end]

Por padrão, o FastComments exibirá seu logotipo e nome no rodapé desses e-mails:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='Parte inferior de um e-mail de verificação mostrando o logotipo e o nome padrão do FastComments no rodapé'; title='Rodapé do E-mail' app-screenshot-end]

Se você estiver nos planos Flex ou Pro, [O nome do remetente, o e-mail e a identidade visual podem ser personalizados](/guide-multiple-sites.html#from-name-email-logo).