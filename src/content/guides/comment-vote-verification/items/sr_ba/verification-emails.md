Када корисник остави коментар или глас, а није пријављен или његов налог није
потврђен, добиће е-поруку у којој ће бити затражено да потврди ту радњу.

Међутим, трудимо се да не спамујемо ваше кориснике е-порукама, и нећемо послати више од једне
потврдне е-поруке по сесији. Погледајте одељак Сесије за више детаља.

Подразумевано, е-поруке за верификацију коментара изгледају овако:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; title='Comment Verification Email' app-screenshot-end]

Подразумевано, е-поруке за верификацију гласова изгледају овако:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; title='Vote Verification Email' app-screenshot-end]

Подразумевано, FastComments ће приказати свој логотип и име у подножју ових е-порука:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; title='Email Footer' app-screenshot-end]

Ако сте на Flex или Pro плану, [Име пошиљаоца, е-пошта и брендирање се могу прилагодити](/guide-multiple-sites.html#from-name-email-logo).