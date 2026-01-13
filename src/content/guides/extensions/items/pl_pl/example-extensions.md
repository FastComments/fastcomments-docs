---
W FastComments tworzymy własne rozszerzenia, korzystając z tego samego API. Możesz przeglądać nieskompresowany kod tych rozszerzeń pod następującymi endpointami:

#### Tryb ciemny

Rozszerzenie Tryb ciemny jest ładowane warunkowo, kiedy wykryta zostanie strona "dark". To rozszerzenie po prostu dodaje trochę CSS do widżetu komentarzy. W ten sposób nie musimy ładować CSS trybu ciemnego, gdy nie jest potrzebny.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.dark.extension.js

#### Moderacja

Gdy bieżący użytkownik jest moderatorem, używamy rozszerzenia moderacji.

To dobry przykład dodawania nasłuchiwaczy zdarzeń opartych na kliknięciach, wykonywania żądań do API, dodawania pozycji do menu komentarza oraz obszarów komentarzy.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.moderation.extension.js

#### Czat na żywo

Rozszerzenie Czat na żywo (w połączeniu z inną konfiguracją i stylizacją) zamienia widżet komentarzy w komponent czatu na żywo.

https://fastcomments.com/js/comment-ui/extensions/live-chat.extension.js

---