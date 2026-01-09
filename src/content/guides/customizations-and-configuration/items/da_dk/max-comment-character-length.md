[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Det maksimale antal tegn, som kan indtastes i kommentarindtastningsfeltet, kan begrænses af parameteren **maxCommentCharacterLength**.

Standardværdien er 2000.

Elementer som billed-URL'er medregnes ikke i længdeberegningen.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

Dette kan tilpasses uden kode på widget-tilpasningssiden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]

---