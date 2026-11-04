Lorsqu'un utilisateur laisse un commentaire ou un vote, et qu'il n'est pas connecté, ou que son compte n'est pas vérifié, il recevra un email lui demandant de vérifier cette action.

Cependant, nous faisons de notre mieux pour ne pas spammer vos utilisateurs avec des emails, et n'envoyons pas plus d'un email de vérification par session. Consultez la section Sessions pour plus de détails.

Par défaut, les emails de vérification de commentaire ressemblent à ceci :

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Corps de l\'email de vérification par défaut citant le commentaire d\'Alexander avec un bouton pour confirmer la publication'; title='Email de vérification de commentaire' app-screenshot-end]

Par défaut, les emails de vérification de vote ressemblent à ceci :

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Email par défaut demandant à Devon de confirmer un vote, montrant le commentaire voté et un bouton de confirmation'; title='Email de vérification de vote' app-screenshot-end]

Par défaut, FastComments affichera son logo et son nom dans le pied de page de ces emails :

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='Bas d\'un email de vérification montrant le logo et le nom FastComments par défaut dans le pied de page'; title='Pied de page de l\'email' app-screenshot-end]

Si vous êtes sur les offres Flex ou Pro, [Le nom de l'expéditeur, l'email et la marque peuvent être personnalisés](/guide-multiple-sites.html#from-name-email-logo).