FastComments は SSO（シングルサインオン）を通じて Drupal のユーザーシステムと統合します。ユーザーはあなたの Drupal サイトにサインインし、モジュールがその身元を自動的に FastComments に渡します。追加のアカウントを作成する必要も、初期同期を実行する必要もありません。

モジュールは 3 つの SSO モードをサポートしており、`Administration > Configuration > Content > FastComments` で設定します。

### なし

SSO なし。ユーザーはゲストとしてコメントするか、FastComments アカウントを作成します。サイトが公開されていてコメントを Drupal ユーザーに紐付ける必要がない場合はこれを使用してください。

### シンプル

サーバー側の検証なしに、Drupal ユーザーの名前、メール、およびアバターを FastComments に渡します。API Secret は不要です。内部用またはリスクの低いサイトに適しています。

### セキュア（推奨）

[HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) を使用して各ユーザーの身元を FastComments 側で検証します。API Secret を設定している場合に使用すべきモードであり、訪問者が他のユーザーになりすますのを防げる唯一のモードです。

ユーザーの身元はユーザーがコメントスレッドを表示するたびに FastComments に渡されます。実行する必要のある初期同期や継続的な同期はありません。

<sup>(任意)</sup> 管理者を [Users & Administrators](https://fastcomments.com/auth/my-account/users) に、モデレーターを [Comment Moderators](https://fastcomments.com/auth/my-account/moderate-comments/moderators) に追加すると、操作性が向上しモデレーターの統計トラッキングが有効になります。

SSO の仕組みを詳しく知りたい場合は、カスタマイズドキュメントの [SSO セクション](/guide-customizations-and-configuration.html#sso) を参照してください。

---