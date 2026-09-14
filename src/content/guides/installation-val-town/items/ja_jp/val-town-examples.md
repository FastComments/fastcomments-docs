---
Four public vals you can remix, each covering one piece of this guide.

**[Blog with comments](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) は、各投稿の下にスレッドがあり、インデックスでコメント数を一括表示するMarkdownブログです。リミックスした瞬間に動作し、1つの環境変数で自分のアカウントを指すようになります。

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) は、訪問者をVal Townアカウントでサインインさせ、そのアイデンティティをウィジェットに渡すので、二度目のログインは不要です。

**[Webhook receiver](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) は、各配信でHMAC署名を検証し、イベントをSQLiteに保存します。テストペイロードに署名して自身に配信するボタンがあり、実際のWebhookを設定する前に検証が成功する様子を確認できます。

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) は、ウィジェット、SSO、REST API、モデレーション、Disqusからの移行をカバーするFastCommentsエージェントスキルのライブラリです。リミックスすると、Val TownのエージェントであるTownieが `skills/` から自動的にスキルを取得するため、チャットにドキュメントを貼り付けなくてもエージェントがコメント機能を設定できるようになります。

同じスキルは、`npx skills add fastcomments/skills` を使って他の場所にもインストールできます。
---