When a user leaves a comment, or vote, and they are not logged in, or their account is
unverified, they will receive an email asking them to verify this action.

しかし、ユーザーにメールでスパムを送らないよう最善を尽くしており、セッションごとに検証メールは1通までに制限しています。詳細は Sessions セクションをご覧ください。

By default, the comment verification emails look like the following:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='デフォルトの検証メール本文で、Alexander のコメントを引用し、投稿を確認するボタンがあります'; title='コメント検証メール' app-screenshot-end]

By default, the vote verification emails look like the following:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Devon に投票の確認を求めるデフォルトメールで、投票されたコメントと確認ボタンが表示されます'; title='投票検証メール' app-screenshot-end]

By default, FastComments will show its logo and name in the footer of these emails:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='検証メールの下部に、フッターにデフォルトの FastComments ロゴと名前が表示されています'; title='メールフッター' app-screenshot-end]

If you are on the Flex or Pro tiers, [The from name, email, and branding can be customized](/guide-multiple-sites.html#from-name-email-logo).