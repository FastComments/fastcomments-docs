When a user leaves a comment, or vote, and they are not logged in, or their account is
unverified, they will receive an email asking them to verify this action.

However, we do our best not to spam your users with emails, and won't send more than one
verification email per session. See the Sessions section for more details.

By default, the comment verification emails look like the following:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Alexander의 댓글을 인용하고 게시물을 확인하는 버튼이 포함된 기본 검증 이메일 본문'; title='댓글 검증 이메일' app-screenshot-end]

By default, the vote verification emails look like the following:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Devon에게 투표를 확인하도록 요청하는 기본 이메일로, 투표된 댓글과 확인 버튼을 표시합니다'; title='투표 검증 이메일' app-screenshot-end]

By default, FastComments will show its logo and name in the footer of these emails:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='검증 이메일 하단에 기본 FastComments 로고와 이름이 푸터에 표시됩니다'; title='이메일 푸터' app-screenshot-end]

If you are on the Flex or Pro tiers, [The from name, email, and branding can be customized](/guide-multiple-sites.html#from-name-email-logo).