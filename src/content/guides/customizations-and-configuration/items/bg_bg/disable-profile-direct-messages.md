[related-parameter-start name = 'disableProfileDirectMessages'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments показва раздел „Директни съобщения“ в потребителските профили, позволявайки на посетителите да изпращат директни съобщения до потребител.

Въпреки това можем да деактивираме този раздел:

[code-example-start config = {disableProfileDirectMessages: true}; linesToHighlight = [6]; title = 'Disable Profile Direct Messages'; code-example-end]

Това може да се направи и без код. На страницата за персонализиране на уиджета, вижте раздела „Деактивиране на директни съобщения“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-direct-messages']; selector = '.disable-profile-direct-messages'; alt='Страница за персонализиране на уиджета с отметка „Disable Direct Messages“ за да се скрие разделът за съобщения в профила'; title='Деактивиране на директни съобщения в профила' app-screenshot-end]