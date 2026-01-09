U FastCommentsu pišemo sopstvene ekstenzije koristeći isti API. Ne-minifikovani kod
ovih ekstenzija možete pogledati na sledećim endpoint-ovima:

#### Tamni režim

Ekstenzija Tamni režim se uslovno učitava kada se detektuje "dark" stranica. Ova ekstenzija jednostavno dodaje
neki CSS komentarskom widgetu. Na taj način ne moramo da učitavamo dark mode CSS kada nije potreban.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.dark.extension.js

#### Moderacija

Kada je trenutni korisnik moderator, koristimo ekstenziju za moderaciju.

Ovo je dobar primer za dodavanje slušača događaja zasnovanih na klikovima, pravljenje API zahteva, dodavanje u meni komentara i oblasti komentara.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.moderation.extension.js

#### Uživo ćaskanje

Ekstenzija Live Chat (u kombinaciji sa dodatnom konfiguracijom i stilizacijom) pretvara komentarski widget u komponentu za ćaskanje uživo.

https://fastcomments.com/js/comment-ui/extensions/live-chat.extension.js