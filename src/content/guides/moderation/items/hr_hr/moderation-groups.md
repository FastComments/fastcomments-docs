---
Moderatori se mogu smjestiti u grupe kako bi moderirali različite stranice ili kategorije sadržaja.

Kada moderator pripada jednoj ili više grupa, vidjet će samo komentare iz tih grupa na stranici Moderiraj komentare.

Na primjer, recimo da vodimo web mjesto koje prikazuje videozapise po kategorijama. Možda želimo imati različite moderatore za videozapise mačaka, pasa i papiga, pa [dodajmo te grupe](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; alt='Popis grupa moderacije s grupama Mačka, Pas i Papiga kreiranim za svaku video kategoriju'; title='Stranica grupa moderacije' app-screenshot-end]

Kada dodamo moderatora, sada imamo mogućnost odabrati jednu ili više grupa kojima će moderator pripadati:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='Obrazac za dodavanje moderatora s odabiračem grupa koji se koristi za dodjeljivanje moderatora jednoj ili više grupa'; title='Dodavanje moderatora i odabir grupe' app-screenshot-end]

Na kraju, komentari moraju biti povezani s jednom ili više grupa kako bi ih vidjeli odgovarajući moderatori.

Ovo se može postaviti [dodavanjem nekih grupa](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) i zatim specificiranjem odgovarajućih `Moderation Group` ID‑ova u widgetu za komentare,
[kao što je opisano ovdje](/guide-customizations-and-configuration.html#moderation-group-ids).