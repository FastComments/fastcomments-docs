FastComments を使用してサイト上でユーザーがコメントすることを禁止する方法は 2 つあります。

最初の方法は、すでにメールアドレスを知っている場合、<a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">禁止ユーザー</a>ページに入力することです。

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Moderate Comments の下にある禁止ユーザーリスト、禁止されたメールアドレスと新しい禁止を追加するボタン'; title='禁止ユーザーページ' app-screenshot-end]

このページは「Moderate Comments」→「Banned Users」からアクセスできます。

ユーザーを禁止する際、タイプを選択できます。Permanent（永久）または Permanent Shadow Ban（永久シャドウバン）のいずれかです：

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='メールフィールドと永久または永久シャドウバンの禁止タイプ選択がある新しい禁止フォーム'; title='ユーザーの禁止' app-screenshot-end]

ユーザーを禁止する 2 番目の方法は、Comment Moderation ページの各コメントに配置されている禁止ボタンをクリックすることです。

禁止ボタンをクリックすると、いくつかのオプションが表示され、禁止タイプと期間を指定できます。

### メールエイリアス

メールでユーザーを禁止する場合、FastComments は自動的に `+` エイリアスを無視します。たとえば、`user+alias@gmail.com` を禁止すると、`user@gmail.com` およびそのアドレスの他の `+` バリエーション（例: `user+other@gmail.com`）も禁止されます。

### シャドウバン

シャドウバンは、ユーザーのコメントや投票が実際には保存されていないにもかかわらず、正常に保存されたように見せる禁止タイプです。特定の状況ではこれが望ましい場合があります。

### IP アドレスによる禁止

テナントがオプトアウトしない限り、FastComments はコメント投稿者の IP アドレスのハッシュ版を保存することで、IP による禁止をサポートします。

### 禁止ユーザーの検索

リストが 1〜2 ページ以上になると、テーブル上部の検索行で絞り込むことができます。

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='検索対象ドロップダウン、マッチドロップダウン、値入力がある禁止ユーザーページの検索行'; title='禁止ユーザーの検索' app-screenshot-end]

3 つのコントロールがあります：

- **Search By** は検索対象のフィールドを選択します：Any Field、Email、Name、Banned By、または Banned For Saying。後の 4 つはテーブルの同名列に対応します。
- **Match** は比較方法を選択します。**Contains** はフィールド内の任意の位置に値があるかを検索し、**Equals** はフィールド全体と一致するかを比較します。
- **Value** は検索するテキストです。

すべてのフィールドは大文字小文字を区別せずに一致するため、`SPAMMER@EXAMPLE.COM` を検索すると、`spammer@example.com` として保存された禁止が見つかります。

知っておくべき点がいくつかあります：

- **Banned For Saying** はユーザーが禁止されたコメントのテキストを検索します。特定のフレーズで禁止されたすべてのユーザーを見つける方法です。
- **Banned By** は禁止を実行したモデレーターの名前を検索します。別のモデレーターの判断をレビューする際に便利です。
- ワイルドカード禁止は `*` と共に保存されるため、`bademail.com` の **Contains** 検索は `*@bademail.com` の禁止を見つけます。
- **Name** は Name 列に表示される名前と一致するため、禁止後に名前を変更したユーザーや、メールアドレスのみで禁止を作成し名前が記録されていなかった場合でもユーザーを見つけられます。禁止時に記録された名前も一致するため、古い名前でも現在の名前でも検索できます。
- **Any Field** はメール、名前、Banned By モデレーター、そして禁止されたコメントテキストをまとめて検索します。

検索はページ URL の一部になるため、他のモデレーターとフィルタ済みリストを他のモデレーションリンクと同様に共有できます。結果ページングは検索を保持し、新しい検索を開始すると最初のページに戻り、**Clear** は全リストに戻ります。