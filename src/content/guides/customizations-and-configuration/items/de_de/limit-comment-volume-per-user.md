Standardmäßig kann jeder Benutzer bis zu `5 comments` in derselben Minute absenden.

Dies wird anhand von user id, anon user id und ip address (hashed) nachverfolgt.

Dies kann ohne Code auf der Seite zur Widget-Anpassung angepasst werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Beachte, dass, wenn du die comment creation API verwendest, du möglicherweise die ursprüngliche `ip` address des Benutzers in der Anfrage an unser backend übergeben möchtest, damit rate limiting angewendet wird
pro Benutzer und nicht global für dein Konto.