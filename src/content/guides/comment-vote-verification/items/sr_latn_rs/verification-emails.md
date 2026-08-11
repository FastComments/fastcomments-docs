Kada korisnik ostavi komentar ili glas, a nije prijavljen ili mu je nalog neverifikovan, dobiće e‑mail koji ga traži da verifikuje ovu radnju.

Međutim, trudimo se da ne spamujemo vaše korisnike e‑mailovima i nećemo poslati više od jednog verifikacionog e‑maila po sesiji. Pogledajte odeljak Sesije za više detalja.

Podrazumevano, verifikacioni e‑mailovi za komentar izgledaju ovako:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Podrazumevano telo verifikacionog e‑maila koje citira komentar Aleksandra uz dugme za potvrdu posta'; title='Verifikacioni e‑mail za komentar' app-screenshot-end]

Podrazumevano, verifikacioni e‑mailovi za glas izgledaju ovako:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Podrazumevani e‑mail koji traži od Devona da potvrdi glas, prikazuje komentar na koji je glasano i dugme za potvrdu'; title='Verifikacioni e‑mail za glas' app-screenshot-end]

Podrazumevano, FastComments će prikazati svoj logo i ime u podnožju ovih e‑mailova:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='Donji deo verifikacionog e‑maila koji prikazuje podrazumevani FastComments logo i ime u podnožju'; title='Podnožje e‑maila' app-screenshot-end]

Ako ste na Flex ili Pro planovima, [Ime pošiljaoca, e‑mail i brendiranje mogu biti prilagođeni](/guide-multiple-sites.html#from-name-email-logo).