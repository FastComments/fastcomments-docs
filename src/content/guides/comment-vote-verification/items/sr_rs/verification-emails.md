Када корисник остави коментар или гласа, а није пријављен или је његов налог
непотврђен, примиће имејл са захтевом да потврди ову радњу.

Међутим, трудимо се да не спамимо ваше кориснике имејловима и нећемо послати више од
једног имејла за потврду по сесији. Погледајте одељак Sessions за више детаља.

Подразумевано, имејлови за потврду коментара изгледају овако:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Подразумевани садржај имејла за потврду са цитатом коментара Александра и дугметом за потврду објаве'; title='Имејл за потврду коментара' app-screenshot-end]

Подразумевано, имејлови за потврду гласа изгледају овако:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Подразумевани имејл који тражи од Девона да потврди глас, приказује коментар на који је гласано и дугме за потврду'; title='Имејл за потврду гласа' app-screenshot-end]

Подразумевано, FastComments приказује свој лого и име у подножју ових имејлова:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='Дно имејла за потврду са подразумеваним логом и именом FastComments у подножју'; title='Подножје имејла' app-screenshot-end]

Ако сте на Flex или Pro пакету, [Име пошиљаоца, имејл и брендирање могу се прилагодити](/guide-multiple-sites.html#from-name-email-logo).