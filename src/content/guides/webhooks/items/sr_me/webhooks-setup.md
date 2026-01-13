Пратите исте кораке за `localhost` као и за production. Убедите се да имате подешене production домене и API Secrets.

Прво, идите на [Webhooks админ](https://fastcomments.com/auth/my-account/manage-data/webhooks). Ово је доступно преко Управљање подацима -> Webhooks.

Страница за конфигурацију изгледа на следећи начин:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration'; cacheBuster = 'v3' app-screenshot-end]

На овој страници можете назначити endpoints за сваку врсту догађаја у вези са коментарима.

За сваку врсту догађаја, обавезно кликните Send Test Payload како бисте били сигурни да сте правилно подесили интеграцију. Погледајте следећи одељак „Тестирање“ за детаље.