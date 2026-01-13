---
U FastCommentsu pišemo vlastite ekstenzije, koristeći isti API. Možete pogledati neminificirani kod ovih ekstenzija na sljedećim endpointima:

#### Tamni način

Ekstenzija Tamni način se uvjetno učitava kada se otkrije stranica označena kao "dark". Ova ekstenzija jednostavno dodaje neke CSS stilove u widget za komentare. Na taj način ne moramo učitavati CSS za tamni način kada on nije potreban.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.dark.extension.js

#### Moderacija

Kada je trenutni korisnik moderator, koristimo ekstenziju za moderaciju.

Ovo je dobar primjer za dodavanje slušača događaja na klik, obavljanje API zahtjeva i dodavanje u izbornik komentara te područja komentara.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.moderation.extension.js

#### Razgovor uživo

Ekstenzija Razgovor uživo (u kombinaciji s drugom konfiguracijom i stilizacijom) pretvara widget za komentare u komponentu chata uživo.

https://fastcomments.com/js/comment-ui/extensions/live-chat.extension.js

---