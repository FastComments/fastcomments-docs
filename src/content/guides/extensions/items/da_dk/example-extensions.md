Hos FastComments skriver vi vores egne udvidelser ved hjælp af det samme API. Du kan se den uminificerede kode
for disse udvidelser på følgende endpoints:

#### Dark Mode

Dark Mode-udvidelsen indlæses betinget, når en "dark" side detekteres. Denne udvidelse tilføjer blot
noget CSS til kommentar-widgetten. På den måde behøver vi ikke indlæse dark mode CSS, når det ikke er nødvendigt.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.dark.extension.js

#### Moderation

Når den aktuelle bruger er en moderator, bruger vi moderationsudvidelsen.

Dette er et godt eksempel på at tilføje klikbaserede event listeners, lave API-anmodninger samt tilføje til kommentarmenuen og kommentarområder.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.moderation.extension.js

#### Live Chat

Live Chat-udvidelsen (i kombination med anden konfiguration og styling) omdanner kommentar-widgetten til en live chat
komponent.

https://fastcomments.com/js/comment-ui/extensions/live-chat.extension.js