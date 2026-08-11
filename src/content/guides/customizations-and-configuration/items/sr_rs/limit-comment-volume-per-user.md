By default, each user can submit up to `5 comments` in the same minute.

This is tracked by user id, anon user id, and ip address (hashed).

This can be customized without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='Polje za maksimalan broj komentara po minutu na stranici za prilagođavanje widgeta, podrazumevano postavljeno na 5'; title='Ograničavanje broja komentara po korisniku' app-screenshot-end]

Note that if you're using the comment creation API may want to pass the user's original `ip` address in the request to our backend so rate limiting is applied
per user and not globally to your account.