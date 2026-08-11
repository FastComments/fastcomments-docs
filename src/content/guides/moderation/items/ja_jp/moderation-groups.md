---
モデレーターは、異なるページやコンテンツのカテゴリを管理するためにグループに配置できます。

モデレーターが1つ以上のグループに所属している場合、そのモデレーターは「コメントのモデレート」ページでそのグループのコメントのみを見ることができます。

例えば、カテゴリ別に動画を表示するサイトを運営しているとします。Cat、Dog、Parrot の動画に対して異なるモデレーターを設定したい場合、[それらのグループを追加しましょう](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups)。

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; alt='各ビデオカテゴリごとに作成された Cat、Dog、Parrot グループを含むモデレーショングループのリスト'; title='モデレーショングループページ' app-screenshot-end]

モデレーターを追加すると、モデレーターが所属する1つ以上のグループを選択するオプションが表示されます：

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='モデレーターを追加するフォームで、モデレーターを1つ以上のグループに割り当てるために使用されるグループセレクタ'; title='モデレーターの追加とグループの選択' app-screenshot-end]

最後に、コメントは1つ以上のグループに紐付けられる必要があり、適切なモデレーターがそれらを見ることができます。

これは、[いくつかのグループを追加する](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups)で設定でき、コメントウィジェットで対応する `Moderation Group` ID を指定します、  
[ここで指示されているように](/guide-customizations-and-configuration.html#moderation-group-ids)。
---