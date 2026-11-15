Sve metrike za koje možete biti ograničeni ograničenjem (rate limited) ili naplaćeni prikazane su na [Billing Analytics stranici](https://fastcomments.com/auth/my-account/analytics/billing).

Neki primeri metrika koje se prate:

- Učitavanja stranica (za tekući mesec)
- API krediti (za tekući mesec)
- Novi sačuvani komentari (za tekući mesec)
- Korisnici tenanta
- SSO korisnici (obični SSO korisnici bez admin/moderator dozvola)
- SSO admini (SSO korisnici sa `isAccountOwner` ili `isAdminAdmin` oznakom - naplaćuju se odvojeno od običnih SSO korisnika)
- SSO moderatori (SSO korisnici sa `isCommentModeratorAdmin` oznakom - naplaćuju se odvojeno od običnih SSO korisnika)
- Moderatori
- Domeni

Za `Creators` i `Pro` planove, ove metrike služe samo kao vodič da biste znali da li ste blizu ili ste dostigli limite svog naloga.

Za `Flex` plan, ova stranica prikazuje raščlanjivanje troškova trenutne upotrebe za **sledeću** fakturu. To olakšava razumevanje kako ćete biti naplaćeni pre nego što se to dogodi. Prethodne fakture možete videti u [Billing History](https://fastcomments.com/auth/my-account/billing-history). Kako se svaki red računa, i kada se faktura naplaćuje, objašnjeno je u [Vodiču za naplatu](/guide-billing.html#how-flex-usage-is-measured).

Metrike `Page Loads`, `API Credits` i `New Comments Stored` mogu biti zastarele do jedan minut. Ostale metrike su u realnom vremenu.

Ove informacije se čuvaju na mesečnom nivou tokom jedne godine, ili dok ne obrišete svoj tenant, u istorijske svrhe, ali UI prikazuje samo tekući period.