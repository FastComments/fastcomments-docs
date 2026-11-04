Når en bruger efterlader en kommentar eller en stemme, og de ikke er logget ind, eller deres konto er
uverificeret, vil de modtage en e-mail, hvor de bliver bedt om at verificere denne handling.

Vi gør dog vores bedste for ikke at spamme dine brugere med e-mails og sender ikke mere end én
verifikations-e-mail pr. session. Se afsnittet Sessions for flere detaljer.

Som standard ser verifikations-e-mails for kommentarer ud som følgende:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Standard verifikations-e-mail, der citerer Alexanders kommentar med en knap til at bekræfte opslaget'; title='Kommentarverifikations-e-mail' app-screenshot-end]

Som standard ser verifikations-e-mails for stemmer ud som følgende:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Standard-e-mail, der beder Devon om at bekræfte en stemme, med den stemte kommentar og en bekræftelsesknap'; title='Stemmeverifikations-e-mail' app-screenshot-end]

Som standard viser FastComments sit logo og navn i bunden af disse e-mails:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='Bunden af en verifikations-e-mail, der viser standard FastComments-logo og -navn i sidefoden'; title='E-mail-sidefod' app-screenshot-end]

Hvis du er på Flex- eller Pro-abonnementerne, kan [afsendernavn, e-mail og branding tilpasses](/guide-multiple-sites.html#from-name-email-logo).