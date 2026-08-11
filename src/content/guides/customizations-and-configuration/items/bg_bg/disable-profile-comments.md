[related-parameter-start name = 'disableProfileComments'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments показва раздел „Profile Comments“ в потребителските профили, позволявайки на посетителите да оставят коментари в профила на някого.

Въпреки това можем да деактивираме този раздел:

[code-example-start config = {disableProfileComments: true}; linesToHighlight = [6]; title = 'Disable Profile Comments'; code-example-end]

Това може да се направи и без код. На страницата за персонализиране на уиджета, вижте раздела „Disable Profile Comments“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-comments']; selector = '.disable-profile-comments'; alt='Widget customization page with the Disable Profile Comments checkbox checked to hide the profile comments tab'; title='Disable Profile Comments' app-screenshot-end]