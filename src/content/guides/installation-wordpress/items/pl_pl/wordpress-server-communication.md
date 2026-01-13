Aby wtyczka działała, token jest zapisywany w bazie danych WordPressa i również na Twoim koncie FastComments. Gdy wtyczka wysyła żądanie do naszych serwerów, dostarcza
ten token.

Wszystkie integracje autoryzowane dla Twojego konta FastComments możesz zobaczyć [tutaj](https://fastcomments.com/auth/my-account/manage-data/integrations).

Cała komunikacja odbywa się przez HTTPS.

Cała komunikacja jest *wychodząca* z Twojego serwera WordPress *do* FastComments.com, włącznie z synchronizacją *z powrotem* do Twojej instalacji WordPress, ponieważ jest ona realizowana
za pomocą [odpytywania](https://en.wikipedia.org/wiki/Polling_(computer_science)) z konfiguracji [cron](https://developer.wordpress.org/plugins/cron/) w Twojej instalacji WordPress.