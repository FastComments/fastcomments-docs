[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

FastComments を使用すると、コメントウィジェット内のすべてのテキストをカスタマイズできます。

送信ボタンのような単一のテキストや、コメントウィジェット全体のすべてのテキストを上書きすることができます。

デフォルトでは、コメントウィジェット内のテキストはユーザーのロケールに基づいて翻訳されます。ただし、ユーザー層が同じロケール/言語を使用していると確信できる場合は、テキストを上書きすることができます。例として：

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'カスタムテキスト'; code-example-end]

すべてのカスタマイズ可能な翻訳は、<a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">こちら</a>の「高度なオプション」タブで確認できます。

しかし、ウィジェットカスタマイズ UI を使用すると、より簡単な方法があります。そこでは、EN_US ロケールでコメントウィジェットに表示されるテキストを見つけて、置き換えを指定するだけです。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='ドロップダウンから選択されたウィジェット文字列と置換テキストフィールドを含むカスタムテキストパネル'; title='カスタムテキスト' app-screenshot-end]

現在、すべての翻訳上書きはすべてのロケールに影響します。

これはウィジェット自身のテキストに関するものです。訪問者が書くコメントを翻訳するには、[Comment Translation](#comment-translation) を参照してください。