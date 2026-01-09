SSO（シングルサインオン）は、ユーザーが別のアカウントを作成することなく FastComments を利用できるようにするための一連の規約です。

匿名コメントを許可していないと仮定すると、FastComments でコメントするにはアカウントが必要です。サインアップは非常に簡単で、ユーザーはコメント時にメールアドレスを入力するだけです。
しかし、それすらも避けたいサイトがあることは理解しています。

### どうやって利用できますか？
現在、すべてのアカウントタイプで SSO にアクセスできます。ただし、SSO ユーザーの最大数はプランによって異なります。他の機能と同様に、Pro プラン以上では直接的な開発サポートが提供されます。

オプションを比較した後、各オプションの詳細を説明します。

### ユーザーとコメントの移行

Disqus のような SSO を提供するプラットフォームから移行する場合、すでにユーザーとそのコメントが存在します。

コメントは移行の一環として、API、Import UI、またはカスタマーサポートによってインポートされます。使用しているプラットフォームを Import UI がサポートしている場合、エラー処理、アバターやメディアの抽出とアップロード、バッチジョブの監視システムが組み込まれているため、Import UI を使用することを推奨します。

ユーザー自体は、コメントスレッドを初めて表示したときに自動的に追加されます。あるいは、API を通じて事前に追加することもできますが、その作業は多くの利点をもたらしません。

コメントがインポートされ、SSO ユーザーが API で手動追加されていない場合、ユーザーが任意のコメントスレッドを初めて表示してアカウントが作成される際に、該当するコメントは自動的にそのユーザーのアカウントに移行されます。その後、ユーザーは元のコメントを管理、編集、削除できるようになります。

自動移行はメールアドレスまたはユーザー名で行われます。Disqus のようにエクスポート時にメールアドレスを提供しないプラットフォームもあるため、その場合はユーザー名にフォールバックします。
- SSO ペイロードに一致するユーザー名とメールアドレスを渡している限り、通知やメンションが機能するように、個々のコメントオブジェクトにそのメールアドレスを追加します。

コメントとユーザーを同時にインポートしたい場合は、API を介してユーザーがインポートされた後に、サポートと連携してコメントをそれぞれのユーザーアカウントに移行してください。

まとめると、移行で最も簡単な手順は次のとおりです：

1. Import comments.
   1. Import UI を `Manage Data -> Imports` で使用している場合、アバターやその他のメディアは自動的に移行されます。
2. Setup Secure or Simple SSO.
3. Let the migration happen per-user automatically when they log in for the first time.
   1. 一般的に、ユーザーが 50,000 件未満のコメントしか持っていない場合、ページ読み込み時間に追加される時間は通常 1 秒未満です。

### WordPress Users
If you're using our <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">WordPress プラグイン</a> then there is no code to write! Simply go to the plugin's Admin page, click SSO Settings, and then Enable.

This will take you to a single-button click wizard which will create your API key, send it over to your WordPress install and turn SSO on. We've consolidated this into a single button click for you.

Note that if you are installing the plugin for the first time you will have to follow up the setup process before you see the admin page with the SSO Settings button.

#### WordPress SSO - Moderators

FastComments WordPress plugin でコメントした際にモデレーターの横に "Moderator" バッジを表示するには、該当ユーザーが FastComments ダッシュボードでも Moderator として追加され、メールアドレスが確認済みである必要がある点に注意してください。

### カスタム統合

カスタム統合の場合、2 つのオプションがあります。

### オプション1 - Secure SSO

With Secure SSO, FastComments knows that the user commenting, voting, and reading comments is a real user on your site.

As long as you create a valid payload, the user will always have a seamless commenting experience.

With Secure SSO, the SSO payload is created **server-side** using HMAC authentication and then passed to the widget on the **client**.

With Secure SSO, the user's account is **completely separate** from the rest of the FastComments user-base. This means if we have two partners
Company A and Company B, each can have an SSO user with the username "Bob".

#### 要件
- サーバーサイド開発に関する基本的な知識。
- 秘密の API キーを扱う際の基本的な知識。
- API 開発またはサーバーサイドレンダリングに関する基本的な知識。

#### 利点
- セキュア。
- シームレスなコメント体験。

#### 欠点
- サーバーサイドの開発を要する。

#### ユーザーデータの更新

Secure SSO では、SSO ユーザーペイロードを渡すたびに、そのユーザーの情報を最新の内容で更新します。例えば、ユーザー名が `X` の場合に SSO ペイロードで `Y` を渡すと、ユーザー名は `Y` になります。

この方法で値を削除したい場合は、それらを `null` に設定してください（`undefined` ではありません）。

#### Secure SSO API

SSO ユーザーと対話するための API も提供しています。詳細は [ドキュメント](/guide-api.html#sso-user-structure) を参照してください。

Note that when using Secure SSO, users are automatically created behind the scenes on page load. You do not have to bulk import your users.

### オプション2 - Simple SSO

The alternative to Secure SSO is to simply pass the user information to the commenting widget.

Providing an email with Simple SSO is not required, however without this their comments will show as "Unverified".

<sup>注：</sup> As of early 2022 usernames with Simple SSO do not need to be unique across all of FastComments.com.

Ideally, Simple SSO should only be picked when developing on a platform that doesn't provide backend access.

#### 要件
- クライアントサイド開発に関する基本的な知識。
- 少なくともユーザーのメールアドレスを把握していること。

#### 利点
- 単純。
- すべてのアクティビティは引き続き検証されます。
- ユーザーが自分のユーザー名やメールアドレスを入力することはありません。

#### 欠点
- クライアント側のペイロードが偽造されて任意のユーザーになり得るため、Secure SSO より安全性は低くなります。

#### Simple SSO API

Simple SSO フローで自動的に作成されたユーザーは `SSOUser` オブジェクトとして保存されます。これらは `SSOUser` API を介してアクセスおよび管理できます。詳細は [ドキュメント](/guide-api.html#sso-user-structure) を参照してください。