---
Kot vsak porazdeljen sistem FastComments potrebuje način za zaklepanje virov in postopkov. Te zaklepe je mogoče spremljati prek končne točke `/locks-in-progress`.

[Na primer, tukaj je končna točka na našem US shardu](https://fastcomments.com/locks-in-progress).

To je lahko koristno za ugotavljanje, zakaj se sistem zatika ali je obremenjen. Če bi SRE morda želel videti, zakaj sistem doživlja visoko obremenitev CPU, lahko preveri to končno točko, da pridobi ime cron opravila, ki ne deluje pravilno.

---