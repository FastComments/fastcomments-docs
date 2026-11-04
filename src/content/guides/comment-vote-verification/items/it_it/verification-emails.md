Quando un utente lascia un commento o un voto, e non ha effettuato l'accesso oppure il suo account non è verificato, riceverà un'email in cui gli viene chiesto di verificare questa azione.

Tuttavia, facciamo del nostro meglio per non inviare email indesiderate ai tuoi utenti e non invieremo più di un'email di verifica per sessione. Consulta la sezione Sessions per maggiori dettagli.

Per impostazione predefinita, le email di verifica dei commenti hanno il seguente aspetto:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Corpo predefinito dell\'email di verifica che riporta il commento di Alexander con un pulsante per confermare la pubblicazione'; title='Email di verifica del commento' app-screenshot-end]

Per impostazione predefinita, le email di verifica dei voti hanno il seguente aspetto:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Email predefinita che chiede a Devon di confermare un voto, mostrando il commento su cui è stato votato e un pulsante di conferma'; title='Email di verifica del voto' app-screenshot-end]

Per impostazione predefinita, FastComments mostrerà il proprio logo e nome nel footer di queste email:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='Parte inferiore di un\'email di verifica che mostra il logo e il nome predefiniti di FastComments nel footer'; title='Footer dell\'email' app-screenshot-end]

Se utilizzi i piani Flex o Pro, [Il nome del mittente, l'email e il branding possono essere personalizzati](/guide-multiple-sites.html#from-name-email-logo).