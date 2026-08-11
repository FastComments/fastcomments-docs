---
Som standard kan hver bruger indsende op til `5 kommentarer` inden for samme minut.

Dette spores via bruger-id, anonym bruger-id og IP-adresse (hashet).

Dette kan tilpasses uden kode på widget-tilpasningssiden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='Maksimalt antal kommentarer pr. minut-feltet på widget-tilpasningssiden, sat til 5 som standard'; title='Begrænsning af kommentarmængde pr. bruger' app-screenshot-end]

Bemærk, at hvis du bruger kommentarskabelses-API'en, vil du måske sende brugerens originale `ip`-adresse i anmodningen til vores backend, så hastighedsbegrænsning anvendes
pr. bruger og ikke globalt for din konto.
---