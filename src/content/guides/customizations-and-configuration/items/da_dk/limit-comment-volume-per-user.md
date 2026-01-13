Som standard kan hver bruger indsende op til `5 comments` i samme minut.

Dette spores ud fra user id, anon user id og ip address (hashed).

Dette kan tilpasses uden kode på widget-tilpasningssiden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Bemærk, at hvis du bruger comment creation API, vil du måske sende brugerens oprindelige `ip` address i anmodningen til vores backend, så rate limiting anvendes
per user og ikke globalt for din account.