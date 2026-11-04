當使用者留言或投票時，如果他們尚未登入，或其帳號未經驗證，他們將會收到一封電子郵件，要求他們驗證此操作。

不過，我們會盡量不對您的使用者發送垃圾郵件，每個工作階段不會發送超過一封驗證郵件。詳情請參閱「工作階段」章節。

預設情況下，留言驗證郵件的外觀如下：

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='預設的驗證郵件內文，引用 Alexander 的留言並附有確認發佈的按鈕'; title='留言驗證郵件' app-screenshot-end]

預設情況下，投票驗證郵件的外觀如下：

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='要求 Devon 確認投票的預設郵件，顯示被投票的留言及確認按鈕'; title='投票驗證郵件' app-screenshot-end]

預設情況下，FastComments 會在這些郵件的頁尾顯示其標誌和名稱：

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='驗證郵件底部，顯示頁尾中預設的 FastComments 標誌和名稱'; title='郵件頁尾' app-screenshot-end]

如果您使用 Flex 或 Pro 方案，[寄件人名稱、電子郵件和品牌資訊可以自訂](/guide-multiple-sites.html#from-name-email-logo)。