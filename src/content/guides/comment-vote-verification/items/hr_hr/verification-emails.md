Kada korisnik ostavi komentar ili glas, a nije prijavljen ili njegov račun nije
potvrđen, primit će e-mail u kojem se traži potvrda te radnje.

Međutim, trudimo se da ne spamamo vaše korisnike e-mailovima i nećemo poslati više od
jedne potvrde po sesiji. Pogledajte odjeljak Sesije za više detalja.

Prema zadanim postavkama, e-mailovi za potvrdu komentara izgledaju ovako:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Zadani sadržaj e-maila za potvrdu s komentarom korisnika Alexandra i gumbom za potvrdu objave'; title='E-mail za potvrdu komentara' app-screenshot-end]

Prema zadanim postavkama, e-mailovi za potvrdu glasa izgledaju ovako:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Zadani e-mail koji Devona traži da potvrdi glas, prikazuje komentirani sadržaj i gumb za potvrdu'; title='E-mail za potvrdu glasa' app-screenshot-end]

Prema zadanim postavkama, FastComments će prikazati svoj logo i naziv u podnožju ovih e-mailova:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='Dno e-maila za potvrdu s zadanim FastComments logom i nazivom u podnožju'; title='Podnožje e-maila' app-screenshot-end]

Ako ste na Flex ili Pro paketu, [Naziv pošiljatelja, e-mail i brendiranje mogu se prilagoditi](/guide-multiple-sites.html#from-name-email-logo).