Kad korisnik ostavi komentar ili glas, a nije prijavljen ili mu je račun nepotvrđen, primit će e‑poruku u kojoj ih se traži da potvrde tu radnju.

Međutim, činimo sve kako bismo korisnike što manje zatrpavali e‑porukama i nećemo poslati više od jedne verifikacijske e‑poruke po sesiji. Pogledajte odjeljak Sesije za više detalja.

Prema zadanim postavkama, verifikacijske e‑poruke za komentare izgledaju ovako:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; title='Comment Verification Email' app-screenshot-end]

Prema zadanim postavkama, verifikacijske e‑poruke za glasanje izgledaju ovako:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; title='Vote Verification Email' app-screenshot-end]

Prema zadanim postavkama, FastComments će prikazati svoj logotip i naziv u podnožju tih e‑poruka:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; title='Email Footer' app-screenshot-end]

Ako ste na Flex ili Pro razini, [Naziv pošiljatelja, adresa e‑pošte i brendiranje mogu se prilagoditi](/guide-multiple-sites.html#from-name-email-logo).