[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Antal kommentarer, der vises øverst i kommentarboksen, kan tilpasses.

Dette kan erstattes med en vilkårlig tekststreng, og værdien **[count]** bliver erstattet med antallet, lokaliseret for brugeren.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Customizing The Comment Count Text'; code-example-end]

Dette kan tilpasses uden kode, på widget-tilpasningssiden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; title='Customizing The Comment Count Text' app-screenshot-end]