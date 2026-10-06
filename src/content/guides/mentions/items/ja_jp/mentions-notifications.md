---
タグ付けされたユーザーには、コメントでタグ付けまたはメンションされたことを知らせるメールが送信されます。

[app-screenshot-start url='/test-e2e/email/comment-user-mention?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"Hey%20%40winrid%20I%20wanted%20you%20to%20see%20this."%2C"commentHTML"%3A"Hey%20<b>%40winrid<%2Fb>%20I%20wanted%20you%20to%20see%20this."%2C"date"%3A1633998787864%2C"pageTitle"%3A"Some%20Page%20Title"%7D&username=winrid&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&INTRO=Hey%20winrid%2C&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&unsubscribeLink=%7B"url"%3A"%2Fauth%2Fmy-account%2Fedit-notifications"%2C"textId"%3A"UNSUBSCRIBE_HERE"%7D&viewCommentUrl=https%3A%2F%2Fexample.com%23fast-comments-jt%3Dsome-db-id&locale=en_us&canReplyByEmail=true&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='通知メール本文は、@winrid メンションが太字で引用されたコメントと、閲覧および配信停止リンクを含みます'; title='ユーザーがメンションされた通知' app-screenshot-end]

通知をオフにするとこれらのメールが送信されなくなり、すべてのメールにはヘッダーが含まれているため、メールクライアントがユーザーにシームレスに配信停止させることができます。

タグ付けされたユーザーは、メールアドレスが登録されていない場合でも、アプリ内通知を受け取ります。

コメントがモデレーターの承認待ちの場合、メンションメールは保留され、コメントが承認され次第送信されます。スパムとしてマークされた場合、削除された場合、または7日以内に承認されなかった場合は送信されません。

メンションされたユーザーにメールが送信されたか、送信されなかった理由を確認するには、[Comment Logs](/guide-moderation.html#comment-logs) をモデレートコメントページから開きます。
---