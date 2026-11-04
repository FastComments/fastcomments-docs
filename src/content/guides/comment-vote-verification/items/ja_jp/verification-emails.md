ユーザーがログインしていない、またはアカウントが未確認の状態でコメントや投票を行うと、その操作を確認するためのメールが送信されます。

ただし、ユーザーにメールでスパムを送らないよう努めており、1セッションにつき確認メールは1通までしか送信しません。詳細については「セッション」のセクションをご覧ください。

デフォルトでは、コメント確認メールは以下のようになります:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Alexanderさんのコメントを引用し、投稿を確認するボタンが付いたデフォルトの確認メール本文'; title='コメント確認メール' app-screenshot-end]

デフォルトでは、投票確認メールは以下のようになります:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='投票されたコメントと確認ボタンを表示し、Devonさんに投票の確認を求めるデフォルトのメール'; title='投票確認メール' app-screenshot-end]

デフォルトでは、FastCommentsはこれらのメールのフッターに自社のロゴと名前を表示します:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='確認メールの下部に、フッターにデフォルトのFastCommentsのロゴと名前が表示されている様子'; title='メールフッター' app-screenshot-end]

FlexまたはProプランをご利用の場合、[送信者名、メールアドレス、ブランディングをカスタマイズできます](/guide-multiple-sites.html#from-name-email-logo)。