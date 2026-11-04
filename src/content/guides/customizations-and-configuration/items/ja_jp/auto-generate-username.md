ユーザーがログインせずにコメントや投票を行う場合、メールアドレスとユーザー名の入力を求められます。

サイトによっては、訪問者に独自のユーザー名を考案してもらうことが、特にモバイルではハードルになることがあります。FastCommentsは、新しい訪問者ごとに中立的なユーザー名を生成し、`BraveOtter4172` のようにユーザー名フィールドに事前入力することができます。

訪問者はそのまま使用することも、任意の名前に変更することもできます。

この機能は、カスタマイズUIの `Generate Usernames Automatically` という設定から有効にできます:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='ウィジェットカスタマイズUIの Generate Usernames Automatically オプション'; title='Generate Usernames Automatically' app-screenshot-end]

#### 動作の仕組み

- 生成される各名前は一意です。既存のアカウントと照合され、その訪問者のブラウザセッション用に予約されるため、2人の訪問者に同じ名前が提案されることはありません。
- 名前は、まだ名前を持っていない訪問者にのみ生成されます。ログインユーザー、SSOユーザー、およびすでにコメントしたことのある訪問者は、既存の名前を保持します。
- [匿名コメント](/guide-customizations-and-configuration.html#allow-anon)の有無にかかわらず機能します。匿名コメントがオフの場合でも、訪問者はメールアドレスを入力しますが、ユーザー名を考える必要はなくなります。
- 以前使用したことのあるメールアドレスを入力した再訪問者は、既存のアカウントと照合され、そのアカウントの名前を保持します。
- [デフォルトユーザー名](/guide-customizations-and-configuration.html#default-username)も設定されている場合、生成された名前が優先されます。