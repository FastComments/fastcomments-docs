[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Максималният брой символи, които могат да бъдат въведени в полето за коментар, може да бъде ограничен чрез параметъра **maxCommentCharacterLength**.

По подразбиране е 2000.

Елементи като URL адреси на изображения не се включват при определянето на дължината.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

Това може да се персонализира без код, на страницата за персонализиране на уиджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]

---