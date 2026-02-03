FastComments は、初めてコメントするユーザーに対して、コメントを送信する前に利用規約への同意を必須にすることができます。

When enabled:
- **匿名ユーザー** はコメントするたびに利用規約のチェックボックスが表示されます
- **認証済みユーザー** は最初のコメント時、または利用規約を更新したときにのみチェックボックスが表示されます

### Enabling Terms of Service

ウィジェットのカスタマイズページに移動し、"Require Terms of Service acceptance" チェックボックスを有効にします:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### Customizing the TOS Text

デフォルトでは、チェックボックスには「I agree to the Terms of Service and Privacy Policy」と両方のドキュメントへのリンクが表示されます。必要に応じてロケールごとにこのテキストをカスタマイズできます:

1. 「Customize text per locale」を選択します
2. ドロップダウンからロケールを選び、カスタムテキストを入力します

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### Updating Your Terms of Service

利用規約を更新する際は、「Last Updated」日付を設定してください。この日付より前に利用規約に同意したユーザーは、再度同意を求められます:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### How It Works

- 利用規約の同意タイムスタンプはユーザーごとおよびコメントごとに保存されます
- ユーザーが利用規約に同意すると、その日付はユーザープロファイル（テナントごと）に記録されます
- 「Last Updated」日付がユーザーの同意日より後の場合、再同意が必要になります
- トラッキングできない匿名ユーザーについては、チェックボックスが毎回のコメント送信時に表示されます

---