FastComments は、モデレーターと管理者向けに、日次、週次、または月次のメールダイジェストをサポートしています。

その頻度は <a href="" target="_blank">ここ</a> で設定できます。

[app-screenshot-start url='/auth/my-account/edit-notifications?demoDigestFrequencyValue=0'; linkUrl='/auth/my-account/edit-notifications'; selector = '.content form'; alt='Digest が毎日、毎週、または毎月配信されるように設定できる Edit Notifications フォーム'; title='Digest 頻度の設定' app-screenshot-end]

コメント全体の統計情報を含めると同時に、レビューが必要な最新の 3 件のコメントも一覧表示されます。

それぞれのコメントには、以下の直接マジックリンクが提供されます。
- コメントを承認する。
- コメントをレビュー済みとしてマークし、返信ページへ移動する。
- コメントをスパムとしてマークする。

各コメントのこれらのリンクは、自動的に認証され、メールからアクションが実行されます。

さらに、ダイジェスト内には「Moderate Comments」ボタンが配置されており、同じ認証を行ってモデレートコメントページへ移動します。

これらのマジックリンクは、一定時間が経過すると期限切れになることに注意してください。

[app-screenshot-start url='/test-e2e/email/tenant-comment-digest?HOST=http%3A%2F%2Flocalhost%3A3001&stats=%7B"hasHistory"%3Atrue%2C"newCommentsCount"%3A10002%2C"hasNewCommentsIncreased"%3Atrue%2C"hasNewCommentsDecreased"%3Afalse%2C"approvedCommentsCount"%3A44%2C"hasApprovedCommentsIncreased"%3Afalse%2C"hasApprovedCommentsDecreased"%3Atrue%2C"spamCommentsCount"%3A21%2C"hasSpamCommentsIncreased"%3Afalse%2C"hasSpamCommentsDecreased"%3Atrue%2C"newUsersCount"%3A30%2C"hasNewUsersIncreased"%3Atrue%2C"hasNewUsersFalse"%3Afalse%7D&BANNER_TEXT=FastComments%20Monthly%20Digest&commentCount=100000&hasCommentsNeedsReview=true&comments=%5B%7B"commenterName"%3A"Devon%20Winrick"%2C"commentHTML"%3A"This%20is%20a%20very%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Devon"%2C"commentHTML"%3A"This%20is%20a%20somewhat%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Bob"%2C"commentHTML"%3A"This%20is%20a%20kind%20of%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Ffastcomments.com%2Fimages%2Funknown-person.png"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%5D&locale=en_us&digestEmail=%7B"tenantId"%3A"tenant-id"%2C"userId"%3A"user-id"%2C"_id"%3A"some-id"%2C"temporaryId"%3A"temporary-id"%7D&API_KEY=T0ph%20123!&rawTemporaryId=xyz'; linkUrl=false; selector = '.content'; alt='コメント統計とレビューが必要な3つのコメントが含まれ、各コメントに承認、返信、スパムリンクが付いた月次ダイジェストメール'; title='ダイジェストメール' app-screenshot-end]

#### Notification Types

FastComments は、モデレーターと管理者に対して複数の種類のメールを送信します。必要に応じて、`Comment Reply` 通知をオプトアウトし、`Edit Notifications` ページで適切なオプションを選択することで `New Comment` 通知は引き続き受け取ることができます。

---