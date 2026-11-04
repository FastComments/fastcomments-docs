Bir kullanıcı yorum bıraktığında veya oy verdiğinde, oturum açmamışsa veya hesabı doğrulanmamışsa, bu eylemi doğrulamalarını isteyen bir e-posta alırlar.

Ancak, kullanıcılarınızı e-postalarla spam yapmamak için elimizden geleni yapıyoruz ve oturum başına birden fazla doğrulama e-postası göndermiyoruz. Daha fazla ayrıntı için Oturumlar bölümüne bakın.

Varsayılan olarak, yorum doğrulama e-postaları aşağıdaki gibi görünür:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Alexander\'ın yorumunu alıntılayan ve gönderiyi onaylamak için bir düğme içeren varsayılan doğrulama e-posta gövdesi'; title='Yorum Doğrulama E-postası' app-screenshot-end]

Varsayılan olarak, oy doğrulama e-postaları aşağıdaki gibi görünür:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Devon\'dan oyu onaylamasını isteyen, oy verilen yorumu ve onay düğmesini gösteren varsayılan e-posta'; title='Oy Doğrulama E-postası' app-screenshot-end]

Varsayılan olarak, FastComments bu e-postaların alt bilgisinde logosunu ve adını gösterir:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='Doğrulama e-postasının alt kısmında alt bilgide varsayılan FastComments logosunu ve adını gösteren bölüm'; title='E-posta Alt Bilgisi' app-screenshot-end]

Flex veya Pro planlarındaysanız, [Gönderen adı, e-posta ve marka özelleştirilebilir](/guide-multiple-sites.html#from-name-email-logo).