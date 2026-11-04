사용자가 로그인하지 않았거나 계정이 인증되지 않은 상태에서 댓글이나 투표를 남기면, 해당 작업을 확인하도록 요청하는 이메일을 받게 됩니다.

다만, 사용자에게 이메일이 스팸처럼 가지 않도록 최선을 다하고 있으며, 세션당 한 번 이상 인증 이메일을 보내지 않습니다. 자세한 내용은 세션(Sessions) 섹션을 참고하세요.

기본적으로 댓글 인증 이메일은 다음과 같이 표시됩니다:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Alexander의 댓글을 인용하고 게시를 확인하는 버튼이 있는 기본 인증 이메일 본문'; title='댓글 인증 이메일' app-screenshot-end]

기본적으로 투표 인증 이메일은 다음과 같이 표시됩니다:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Devon에게 투표를 확인하도록 요청하는 기본 이메일로, 투표된 댓글과 확인 버튼이 표시됨'; title='투표 인증 이메일' app-screenshot-end]

기본적으로 FastComments는 이러한 이메일의 하단에 자체 로고와 이름을 표시합니다:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='인증 이메일 하단에 기본 FastComments 로고와 이름이 푸터에 표시된 모습'; title='이메일 푸터' app-screenshot-end]

Flex 또는 Pro 플랜을 사용 중이라면, [발신자 이름, 이메일, 브랜딩을 사용자 지정할 수 있습니다](/guide-multiple-sites.html#from-name-email-logo).