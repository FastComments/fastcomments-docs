---
[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Kommentarantallet, der vises øverst i kommentarfunktionen, kan tilpasses.

Dette kan erstattes med en vilkårlig streng, og værdien **[count]** vil blive erstattet med antalsværdien, lokalt for brugeren.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Tilpasning af kommentarantalleteksten'; code-example-end]

Dette kan tilpasses uden kode på siden for widgettilpasning:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='Tekstfelt for kommentarantal på siden for widgettilpasning, hvor [count] erstattes med den aktuelle total'; title='Tilpasning af kommentarantalleteksten' app-screenshot-end]

---