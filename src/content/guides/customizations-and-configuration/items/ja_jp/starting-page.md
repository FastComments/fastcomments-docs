[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

コメントを取得してレンダリングする際、コメントウィジェットはどのページから開始するかを知る必要があります。デフォルトでは最初のページから始まり、そのページのみがレンダリングされます。

必要に応じて、描画する正確なページを設定 *startingPage* としてコメントウィジェットに渡すことができます。

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

ページ番号は0から始まることに注意してください。そのため、上の例は2ページ目をレンダリングします。

---