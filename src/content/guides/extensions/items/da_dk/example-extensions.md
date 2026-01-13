Hos FastComments skriver vi vores egne udvidelser ved hjælp af det samme API. Du kan se den ikke-minificerede kode for disse udvidelser på følgende endepunkter:

#### Dark Mode

Dark Mode-udvidelsen indlæses betinget, når en "dark" side opdages. Denne udvidelse tilføjer blot noget CSS til kommentar-widget'en. På denne måde behøver vi ikke at indlæse dark mode CSS, når det ikke er nødvendigt.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.dark.extension.js

#### Moderation

Når den aktuelle bruger er moderator, bruger vi moderation-udvidelsen.

Dette er et godt eksempel på at tilføje klikbaserede event-lyttere, lave API-forespørgsler og tilføje elementer til kommentarmenuen og kommentarområderne.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.moderation.extension.js

#### Live Chat

Live Chat-udvidelsen (i kombination med anden konfiguration og styling) gør kommentar-widget'en til en live chat-komponent.

https://fastcomments.com/js/comment-ui/extensions/live-chat.extension.js