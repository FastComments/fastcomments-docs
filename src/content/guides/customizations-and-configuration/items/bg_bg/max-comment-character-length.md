[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Максималният брой символи, разрешени за въвеждане в полето за коментар, може да бъде ограничен чрез параметъра **maxCommentCharacterLength**.

По подразбиране е 2000.

Не се включват неща като URL адреси на изображения в определянето на дължината.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Ограничаване дължината на коментара'; code-example-end]

Това може да се персонализира без код, на страницата за персонализиране на уиджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='Поле за максимален размер на коментара на страницата за персонализиране на уиджета, използвано за ограничаване на броя символи, които може да съдържа коментарът'; title='Ограничаване дължината на коментара' app-screenshot-end]

---