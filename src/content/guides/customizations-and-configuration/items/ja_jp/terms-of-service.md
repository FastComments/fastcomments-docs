---
FastCommentsでは、初めてコメントするユーザーに対して、コメントを送信する前に利用規約への同意を必須にすることができます。

When enabled:
- **Anonymous users** will see a TOS checkbox every time they comment
- **Authenticated users** will see the checkbox only on their first comment, or when you update your TOS

### 設定

ウィジェットのカスタマイズページに移動し、「Require Terms of Service acceptance」チェックボックスを有効にします。 有効にすると、以下のオプションが表示されます:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **TOS Text Mode**: デフォルトでは、チェックボックスには両方のドキュメントへのリンク付きで "I agree to the Terms of Service and Privacy Policy" と表示されます。 "Customize text per locale" を選択すると、各言語ごとに独自のテキストを指定できます。
- **TOS Last Updated Date**: 利用規約を更新したときは、この日付を設定してください。この日付より前に同意したユーザーは再度同意が必要になります。

### 仕組み

- 利用規約同意のタイムスタンプはユーザーごと・コメントごとに保存されます
- ユーザーが利用規約に同意すると、その日付はユーザープロフィール（テナントごと）に記録されます
- 設定した「最終更新日」がユーザーの同意日より後であれば、再度同意が必要になります
- 追跡できない匿名ユーザーについては、チェックボックスが各コメント送信時に表示されます

---