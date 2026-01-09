---
[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

FastComments では、コメントウィジェット内のすべてのテキストをカスタマイズできます。

送信ボタンなど単一のテキストだけを上書きすることも、コメントウィジェット全体のすべてのテキストを上書きすることもできます。

デフォルトでは、コメントウィジェットのテキストはユーザーのロケールに基づいて翻訳されます。ただし、ユーザーベースが同じロケール/言語を使用していると確信している場合は、テキストを上書きすることができます。例えば:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

すべてのカスタマイズ可能な翻訳は、<a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">こちら</a>の「詳細オプション」タブで確認できます。

しかし、ウィジェットカスタマイズUIを使ったより簡単な方法があります。そこで、コメントウィジェットに表示される EN_US ロケールのテキストを見つけて、置き換えを指定できます。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

現在、翻訳の上書きはすべてのロケールに対して適用されます。

---