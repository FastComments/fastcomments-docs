Bij FastComments schrijven we onze eigen extensies, met gebruik van dezelfde API. Je kunt de niet-gecomprimeerde code
voor deze extensies bekijken op de volgende eindpunten:

#### Donkere modus

De Donkere modus-extensie wordt conditioneel geladen wanneer een "dark"-pagina wordt gedetecteerd. Deze extensie voegt eenvoudig wat CSS toe aan de reactie-widget. Op deze manier hoeven we de CSS voor donkere modus niet te laden wanneer dat niet nodig is.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.dark.extension.js

#### Moderatie

Wanneer de huidige gebruiker moderator is, gebruiken we de moderatie-extensie.

Dit is een goed voorbeeld van het toevoegen van klik-gebaseerde event listeners, het doen van API-aanvragen, en het toevoegen aan het reactiemenu en de reactiegebieden.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.moderation.extension.js

#### Livechat

De Livechat-extensie (in combinatie met andere configuratie en styling) verandert de reactie-widget in een livechatcomponent.

https://fastcomments.com/js/comment-ui/extensions/live-chat.extension.js