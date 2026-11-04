Когато потребител остави коментар или глас и не е влязъл в системата, или акаунтът му не е потвърден, той ще получи имейл с искане да потвърди това действие.

Въпреки това се стараем да не спамим вашите потребители с имейли и няма да изпратим повече от един имейл за потвърждение на сесия. Вижте раздела Sessions за повече подробности.

По подразбиране имейлите за потвърждение на коментари изглеждат по следния начин:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Имейл за потвърждение по подразбиране, цитиращ коментара на Alexander с бутон за потвърждение на публикацията'; title='Имейл за потвърждение на коментар' app-screenshot-end]

По подразбиране имейлите за потвърждение на гласуване изглеждат по следния начин:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Имейл по подразбиране, който моли Devon да потвърди гласуване, показващ коментара, за който е гласувано, и бутон за потвърждение'; title='Имейл за потвърждение на гласуване' app-screenshot-end]

По подразбиране FastComments ще покаже своето лого и име в долния колонтитул на тези имейли:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='Долна част на имейл за потвърждение, показващ логото и името на FastComments по подразбиране в долния колонтитул'; title='Долен колонтитул на имейла' app-screenshot-end]

Ако сте на тарифите Flex или Pro, [Името на подателя, имейлът и брандингът могат да бъдат персонализирани](/guide-multiple-sites.html#from-name-email-logo).