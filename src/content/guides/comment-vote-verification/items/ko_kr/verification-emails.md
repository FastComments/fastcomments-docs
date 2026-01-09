사용자가 댓글이나 투표를 남기고 로그인하지 않았거나 계정이 확인되지 않은 경우, 이 작업을 확인해 달라는 이메일을 받게 됩니다.

다만 사용자에게 스팸성 이메일을 과도하게 발송하지 않도록 최선을 다하며, 세션당 확인 이메일을 한 번만 전송합니다. 자세한 내용은 Sessions 섹션을 참조하세요.

기본적으로 댓글 확인 이메일은 다음과 같습니다:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; title='Comment Verification Email' app-screenshot-end]

기본적으로 투표 확인 이메일은 다음과 같습니다:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; title='Vote Verification Email' app-screenshot-end]

기본적으로 FastComments는 이 이메일들의 푸터에 로고와 이름을 표시합니다:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; title='Email Footer' app-screenshot-end]

Flex 또는 Pro 요금제를 사용 중인 경우, [발신자 이름, 이메일 및 브랜딩을 사용자 지정할 수 있습니다](/guide-multiple-sites.html#from-name-email-logo).