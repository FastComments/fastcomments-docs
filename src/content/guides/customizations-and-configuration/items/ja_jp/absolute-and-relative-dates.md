[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

デフォルトでは、ローカライズされた相対日付が使用されます。例えば、最近投稿されたコメントの横には「11分前」のように表示されることがあります。

この相対日付形式を維持しつつ、同時に完全な日付も併せて表示したい場合は、このパラメータを true に設定します。 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

これはコードなしで、ウィジェットのカスタマイズページの「詳細オプション」でカスタマイズできます。UIでこのオプションを表示するには、まず Absolute Dates を有効にする必要があります。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]

---