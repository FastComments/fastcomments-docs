Wanneer een gebruiker een reactie plaatst of stemt, en niet is ingelogd, of zijn account is niet geverifieerd, ontvangt hij een e-mail met het verzoek deze actie te verifiëren.

We doen er echter alles aan om uw gebruikers niet te spammen met e-mails, en sturen niet meer dan één verificatie-e-mail per sessie. Zie de sectie Sessies voor meer details.

Standaard zien de verificatie-e-mails voor reacties er als volgt uit:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Standaard verificatie-e-mail met de reactie van Alexander en een knop om de post te bevestigen'; title='Verificatie-e-mail reactie' app-screenshot-end]

Standaard zien de verificatie-e-mails voor stemmen er als volgt uit:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Standaard e-mail die Devon vraagt een stem te bevestigen, met de stemde reactie en een bevestigingsknop'; title='Verificatie-e-mail stem' app-screenshot-end]

Standaard toont FastComments zijn logo en naam in de voettekst van deze e-mails:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='Onderkant van een verificatie-e-mail met het standaard FastComments logo en naam in de voettekst'; title='E-mail voettekst' app-screenshot-end]

Als u op het Flex- of Pro-abonnement zit, kan [de afzender naam, e-mail en branding worden aangepast](/guide-multiple-sites.html#from-name-email-logo).