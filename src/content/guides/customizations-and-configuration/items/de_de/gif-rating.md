---
[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Standardmäßig setzt das FastComments-Kommentar-Widget eine `gif rating` von `pg`.

Verfügbare Optionen sind `g`, `pg`, `pg-13` und `r`.

Dies kann im Code oder über die UI eingestellt werden. Im Code können wir es wie folgt tun:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

In der UI finden Sie dies unter `Gif Picker Rating`, sofern `Disable Image Uploads?` nicht aktiviert ist.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]

---