当用户发表评论或投票时，如果他们未登录或其账户未验证，他们将收到一封要求验证此操作的电子邮件。

不过，我们会尽力避免向您的用户发送垃圾邮件，每个会话最多只发送一封验证邮件。详情请参阅“会话（Sessions）”部分。

默认情况下，评论验证邮件如下所示：

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='默认验证邮件正文，引用了 Alexander 的评论并带有确认发布的按钮'; title='评论验证邮件' app-screenshot-end]

默认情况下，投票验证邮件如下所示：

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='要求 Devon 确认投票的默认邮件，显示了被投票的评论和确认按钮'; title='投票验证邮件' app-screenshot-end]

默认情况下，FastComments 会在这些邮件的页脚中显示其徽标和名称：

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='验证邮件底部，页脚中显示默认的 FastComments 徽标和名称'; title='邮件页脚' app-screenshot-end]

如果您使用的是 Flex 或 Pro 套餐，[发件人名称、电子邮件和品牌信息可以进行自定义](/guide-multiple-sites.html#from-name-email-logo)。