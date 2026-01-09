Когато потребител остави коментар, или глас, и не е влязъл в системата, или акаунтът му не е
потвърден, той ще получи имейл с молба да потвърди това действие.

Въпреки това, ние се стараем да не пращаме спам на вашите потребители и няма да изпращаме повече от един
имейл за потвърждение на сесия. Вижте секцията Сесии за повече подробности.

По подразбиране, имейлите за потвърждение на коментар изглеждат по следния начин:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; title='Comment Verification Email' app-screenshot-end]

По подразбиране, имейлите за потвърждение на глас изглеждат по следния начин:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; title='Vote Verification Email' app-screenshot-end]

По подразбиране, FastComments ще показва логото и името си в долния колонтитул на тези имейли:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; title='Email Footer' app-screenshot-end]

Ако сте на план Flex или Pro, [Името на подателя, имейлът и брандирането могат да бъдат персонализирани](/guide-multiple-sites.html#from-name-email-logo).