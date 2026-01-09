Bir kullanıcı yorum bıraktığında veya oy verdiğinde ve giriş yapmamışsa veya hesapları
doğrulanmamışsa, bu işlemi doğrulamaları için bir e-posta alırlar.

Ancak, kullanıcılarınızı e-postalarla spamlememeye özen gösteriyoruz ve oturum başına birden fazla
doğrulama e-postası göndermeyiz. Daha fazla ayrıntı için Oturumlar bölümüne bakın.

Varsayılan olarak, yorum doğrulama e-postaları aşağıdaki gibi görünür:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; title='Comment Verification Email' app-screenshot-end]

Varsayılan olarak, oy doğrulama e-postaları aşağıdaki gibi görünür:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; title='Vote Verification Email' app-screenshot-end]

Varsayılan olarak, FastComments bu e-postaların altbilgisinde logosunu ve adını gösterir:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; title='Email Footer' app-screenshot-end]

Eğer Flex veya Pro planlarındaysanız, [Gönderen adı, e-posta ve markalama özelleştirilebilir](/guide-multiple-sites.html#from-name-email-logo).