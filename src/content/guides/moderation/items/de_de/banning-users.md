Es gibt zwei Möglichkeiten, Benutzer am Kommentieren auf Ihrer Website mit FastComments zu sperren.

Die erste ist: Wenn Sie bereits ihre E-Mail kennen, können Sie diese auf der <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">Gesperrte Benutzer</a>-Seite eingeben.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Auf diese Seite kann über Kommentare moderieren -> Gesperrte Benutzer zugegriffen werden

Wenn wir einen Benutzer sperren, können wir einen Typ auswählen, entweder 'Permanent' oder 'Permanent Shadow Ban':

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

Die zweite Möglichkeit, einen Benutzer zu sperren, besteht darin, den Sperr-Button zu klicken, der an jedem Kommentar auf der Kommentar-Moderation-Seite angebracht ist.

Wenn wir den Sperr-Button klicken, werden Ihnen einige Optionen angezeigt, mit denen Sie den Sperrtyp und die Dauer festlegen können.

### E-Mail-Aliase

Beim Sperren eines Benutzers per E-Mail ignoriert FastComments automatisch `+` Aliase. Zum Beispiel führt das Sperren von `user+alias@gmail.com` auch dazu, dass `user@gmail.com` und jede andere `+`-Variante dieser Adresse, wie `user+other@gmail.com`, gesperrt werden.

### Shadow Bans

Ein Shadow-Ban ist eine Art Sperre, bei der es so aussieht, als wäre der Kommentar oder die Abstimmung des Benutzers erfolgreich gespeichert worden, obwohl dies tatsächlich nicht der Fall ist. Dies kann in bestimmten Situationen erwünscht sein.

### Sperren per IP-Adresse

Sofern ein Mandant nicht widerspricht, unterstützt FastComments das Sperren über IP, indem eine gehashte Version der IP-Adresse des Kommentators gespeichert wird.