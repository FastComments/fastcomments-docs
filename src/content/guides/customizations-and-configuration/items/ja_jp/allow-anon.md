---
デフォルトでは、FastComments はコメントする際にメールアドレスの入力を必須とします。メールアドレスが有効である必要はありませんが、ユーザーが送信されたリンクをクリックするまで、コメントには「未確認コメント」ラベルが表示されます。

ただし、メールアドレスの必須設定は解除できます。メール入力フィールドは引き続き表示されますが、必須ではなくなります。

この設定はウィジェットカスタマイズ UI から構成できます：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.allow-anonymous-comments'; alt='ウィジェットカスタマイズ UI の匿名コメントオプションで、メールフィールドを任意にします'; title='匿名コメントの有効化' app-screenshot-end]

ユーザー名は依然として必須です。そのステップも省略したい場合は、全員が共有する[デフォルトのユーザー名を設定](/guide-customizations-and-configuration.html#default-username)するか、FastComments に各訪問者ごとに[ユニークなユーザー名を自動生成](/guide-customizations-and-configuration.html#auto-generate-username)させることができます。

---