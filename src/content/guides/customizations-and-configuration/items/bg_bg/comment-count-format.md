---
[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Броят на коментарите, показван в горната част на коментарния уиджет, може да бъде персонализиран.

Това може да бъде заменено с произволен текст, а стойността **[count]** ще бъде заменена с числото, локализирано за потребителя.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Customizing The Comment Count Text'; code-example-end]

Това може да бъде персонализирано без писане на код, чрез страницата за персонализиране на уиджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; title='Customizing The Comment Count Text' app-screenshot-end]


---