[related-parameter-start name = 'disableProfileComments'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments ще покаже раздел "Профилни коментари" в потребителските профили, което позволява на посетителите да оставят коментари в профила на някого.

Можем обаче да деактивираме този раздел:

[code-example-start config = {disableProfileComments: true}; linesToHighlight = [6]; title = 'Disable Profile Comments'; code-example-end]

Това може да се направи и без код. На страницата за персонализиране на уиджета вижте секцията "Деактивиране на профилните коментари".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-comments']; selector = '.disable-profile-comments'; title='Disable Profile Comments' app-screenshot-end]