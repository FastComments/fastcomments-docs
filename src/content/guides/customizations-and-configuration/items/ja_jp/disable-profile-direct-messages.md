[related-parameter-start name = 'disableProfileDirectMessages'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はユーザープロフィールに「ダイレクトメッセージ」タブを表示し、訪問者がユーザーに直接メッセージを送信できるようにします。

ただし、このタブを無効化することができます:

[code-example-start config = {disableProfileDirectMessages: true}; linesToHighlight = [6]; title = 'Disable Profile Direct Messages'; code-example-end]

コードを使用せずにも設定できます。ウィジェットカスタマイズページの「ダイレクトメッセージを無効化」セクションをご覧ください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-direct-messages']; selector = '.disable-profile-direct-messages'; alt='ウィジェットカスタマイズページで「ダイレクトメッセージを無効化」チェックボックスがオンになって、プロフィールメッセージタブが非表示になる状態'; title='プロフィールのダイレクトメッセージを無効化' app-screenshot-end]