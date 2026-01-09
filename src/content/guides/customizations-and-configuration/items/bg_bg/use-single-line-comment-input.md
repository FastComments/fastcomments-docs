[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments позволява на потребителя да въведе коментар с толкова редове, колкото желае, до зададения по подразбиране лимит на символите.

Въпреки това може да е желателно да ограничите потребителя да въвежда само един ред текст. Някои примерни случаи на употреба включват онлайн наддаване или чат на живо, за които FastComments
може да бъде използван.

Активираме флага **useSingleLineCommentInput** по следния начин:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Това може да се направи и без код. В страницата за персонализиране на джаджата вижте секцията "Активиране на едноредово въвеждане на коментари".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Обърнете внимание, че коментарите на всяка страница за всяка посока на сортиране са предварително изчислени, така че всички посоки на сортиране имат еднаква производителност.