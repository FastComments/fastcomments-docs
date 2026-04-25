---
Modul priloži več blokov, ki jih lahko postavite iz `Structure > Block layout` (`/admin/structure/block`).

- **FastComments Widget** - Glavni pripomoček za komentiranje. Samodejno zazna trenutno entiteto. Preskočil bo entitete, ki imajo že pripeto FastComments polje, zato na isti strani ne boste videli podvojenih pripomočkov.
- **FastComments Live Chat** - Klepet v realnem času s pretočnim prenosom. Lahko ga postavite ob polje za komentarje na isti strani.
- **FastComments Collab Chat** - Označevanje besedila z opombami in razprava.
- **FastComments Image Chat** - Opombe na podlagi koordinat na slikah. Obiskovalci kliknejo sliko, da pustijo komentarje, vezane na določene lokacije.
- **FastComments Recent Comments** - Prikaže nedavne komentarje na vašem spletišču. Število je nastavljivo na bloku.
- **FastComments Top Pages** - Prikaže strani na vašem spletišču z največ komentarji.

Vsebinski bloki (Live Chat, Collab Chat, Image Chat) samodejno zaznavajo trenutno entiteto in se, na straneh brez entitete, zanašajo na identifikator, osnovan na poti. To pomeni, da delujejo na taxonomy pages, views in custom routes brez dodatne nastavitve.

---