[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Броят на коментарите, показван в горната част на уиджета за коментари, може да бъде персонализиран.

Това може да бъде заменено с произволен низ, а стойността **[count]** ще бъде заменена с броя, локализиран за потребителя.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Персонализиране на текста за брой коментари'; code-example-end]

Това може да бъде персонализирано без код, на страницата за персонализиране на уиджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='Поле за текст на брой коментари на страницата за персонализиране на уиджета, където [count] се заменя с текущия общ брой'; title='Персонализиране на текста за брой коментари' app-screenshot-end]