[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments позволява на потребителя да въведе коментар с толкова редове, колкото желае, до зададения лимит на знаците.

Въпреки това, може да е желателно да се ограничи потребителят да въвежда само един ред текст. Примери за употреба включват онлайн наддаване или жив чат, за които FastComments може да се използва.

Активираме флага **useSingleLineCommentInput** по следния начин:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Това може да се направи и без код. На страницата за персонализиране на уиджета, вижте секцията „Enable Single-Line Comment Input“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='Квадратчето за включване на едноредов вход за коментари е отметнато в страницата за персонализиране на уиджета, ограничавайки въвеждането до един ред'; title='Активиране на вход за едноредов коментар' app-screenshot-end]

Обърнете внимание, че коментарите на всяка страница за всяка посока на сортиране са предварително изчислени, така че всички посоки на сортиране имат еднаква производителност.