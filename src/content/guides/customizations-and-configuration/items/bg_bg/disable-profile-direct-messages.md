[related-parameter-start name = 'disableProfileDirectMessages'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments ще показва раздела "Директни съобщения" в профилите на потребителите, позволявайки на посетителите да изпращат директни съобщения на потребител.

Въпреки това, можем да деактивираме този раздел:

[code-example-start config = {disableProfileDirectMessages: true}; linesToHighlight = [6]; title = 'Disable Profile Direct Messages'; code-example-end]

Това може да се направи и без код. На страницата за персонализиране на уиджета вижте секцията "Деактивиране на директни съобщения".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-direct-messages']; selector = '.disable-profile-direct-messages'; title='Disable Profile Direct Messages' app-screenshot-end]