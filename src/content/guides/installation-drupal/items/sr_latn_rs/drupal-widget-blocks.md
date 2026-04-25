The module ships several blocks you can place from `Structure > Block layout` (`/admin/structure/block`).

- **FastComments Widget** - Glavni widget za komentare. Automatski detektuje trenutni entity. Preskočiće entitete koji već imaju pridružen FastComments field, tako da nećete videti duplirane widget-e na istoj stranici.
- **FastComments Live Chat** - Chat koji radi u realnom vremenu. Može se postaviti pored comment field na istoj stranici.
- **FastComments Collab Chat** - Anotacija i diskusija na izabranom tekstu.
- **FastComments Image Chat** - Anotacije zasnovane na koordinatama na slikama. Posetioci kliknu na sliku da ostave komentare vezane za određene lokacije.
- **FastComments Recent Comments** - Prikazuje najnovije komentare širom vašeg sajta. Broj je konfigurabilan na bloku.
- **FastComments Top Pages** - Prikazuje stranice na vašem sajtu sa najviše komentara.

The content-centric blocks (Live Chat, Collab Chat, Image Chat) auto-detect the current entity, and fall back to a path-based identifier on non-entity pages. That means they work on taxonomy pages, views, and custom routes without any extra setup.