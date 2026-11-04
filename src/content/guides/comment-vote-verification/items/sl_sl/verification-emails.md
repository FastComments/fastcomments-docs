Ko uporabnik pusti komentar ali glas, in ni prijavljen, ali pa njegov račun ni potrjen, bo prejel e-poštno sporočilo, ki ga prosi, da to dejanje potrdi.

Vendar pa se trudimo, da vaših uporabnikov ne zasipamo z e-poštnimi sporočili, zato ne pošljemo več kot enega potrditvenega e-poštnega sporočila na sejo. Za več podrobnosti glejte razdelek Seje.

Privzeto potrditvena e-poštna sporočila o komentarjih izgledajo takole:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Privzeto telo potrditvenega e-poštnega sporočila, ki navaja komentar uporabnika Alexandra z gumbom za potrditev objave'; title='Potrditveno e-poštno sporočilo o komentarju' app-screenshot-end]

Privzeto potrditvena e-poštna sporočila o glasovanju izgledajo takole:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Privzeto e-poštno sporočilo, ki uporabnika Devona prosi za potrditev glasu, prikazuje komentar, na katerega je bilo glasovano, in gumb za potrditev'; title='Potrditveno e-poštno sporočilo o glasovanju' app-screenshot-end]

Privzeto FastComments prikazuje svoj logotip in ime v nogi teh e-poštnih sporočil:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='Spodnji del potrditvenega e-poštnega sporočila, ki prikazuje privzeti logotip in ime FastComments v nogi'; title='Noga e-poštnega sporočila' app-screenshot-end]

Če ste na paketih Flex ali Pro, [Ime pošiljatelja, e-poštni naslov in blagovna znamka lahko prilagodite](/guide-multiple-sites.html#from-name-email-logo).