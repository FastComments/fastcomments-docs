When a user leaves a comment, or vote, and they are not logged in, or their account is
unverified, they will receive an email asking them to verify this action.

However, we do our best not to spam your users with emails, and won't send more than one
verification email per session. See the Sessions section for more details.

By default, the comment verification emails look like the following:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='默认的验证电子邮件正文，引用 Alexander\'s 评论，并带有确认帖子的按钮'; title='评论验证邮件' app-screenshot-end]

By default, the vote verification emails look like the following:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='默认电子邮件，要求 Devon 确认投票，显示被投票的评论以及确认按钮'; title='投票验证邮件' app-screenshot-end]

By default, FastComments will show its logo and name in the footer of these emails:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='验证电子邮件底部，显示默认的 FastComments 徽标和页脚中的名称'; title='电子邮件页脚' app-screenshot-end]

If you are on the Flex or Pro tiers, [发件人名称、电子邮件和品牌可以自定义](/guide-multiple-sites.html#from-name-email-logo).