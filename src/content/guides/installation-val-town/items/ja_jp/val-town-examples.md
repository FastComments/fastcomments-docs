リミックスできる公開valが4つあり、それぞれこのガイドの一部をカバーしています。

**[コメント付きブログ](https://www.val.town/x/fastcomments/blog-with-comments)** ([ライブ](https://fastcomments-blog.val.run)) は、各投稿の下にコメントスレッドがあり、インデックスページにはコメント数が一括表示されるMarkdownブログです。リミックスした瞬間に動作し、環境変数を1つ設定するだけで自分のアカウントに向けられます。

**[SSOデモ](https://www.val.town/x/fastcomments/sso-demo)** ([ライブ](https://fastcomments-sso.val.run)) は、訪問者をVal Townアカウントでサインインさせ、そのIDをウィジェットに渡すため、2度目のログインは不要です。

**[Webhookレシーバー](https://www.val.town/x/fastcomments/webhook-receiver)** ([ライブ](https://fastcomments-webhooks.val.run)) は、すべての配信でHMAC署名を検証し、イベントをSQLiteに保存します。テストペイロードに署名して自分自身に配信するボタンが付いているため、実際のwebhookを設定する前に検証が成功する様子を確認できます。

**[エージェントスキル](https://www.val.town/x/fastcomments/skills)** ([ライブ](https://fastcomments-skills.val.run)) は、ウィジェット、SSO、REST API、モデレーション、Disqusからの移行をカバーするFastCommentsエージェントスキルのライブラリです。リミックスすると、Val TownのエージェントであるTownieが`skills/`から自動的にスキルを取り込むため、ドキュメントをチャットに貼り付けることなく、エージェントがコメントの設定方法を把握できます。

同じスキルは`npx skills add fastcomments/skills`を使って他の環境にもインストールできます。