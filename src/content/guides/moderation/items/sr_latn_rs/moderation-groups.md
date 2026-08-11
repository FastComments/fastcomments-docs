Moderatori se mogu smestiti u grupe kako bi moderirali različite stranice ili kategorije sadržaja.

Kada moderator pripada jednoj ili više grupa, videće samo komentare iz tih grupa na stranici **Moderiraj komentare**.

Na primer, recimo da vodimo sajt koji prikazuje video zapise po kategorijama. Možda želimo da imamo različite moderatore za video zapise mačaka, pasa i papagaja, pa [dodajmo te grupe](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; alt='Lista grupa moderacije sa grupama Mačka, Pas i Papagaj kreiranim za svaku video kategoriju'; title='Stranica grupa moderacije' app-screenshot-end]

Kada dodamo moderatora, sada imamo opciju da izaberemo jednu ili više grupa kojima će moderator pripadati:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='Forma za dodavanje moderatora sa selektorom grupa koji se koristi za dodeljivanje moderatora jednoj ili više grupa'; title='Dodavanje moderatora i izbor grupe' app-screenshot-end]

Na kraju, komentari moraju biti povezani sa jednom ili više grupa kako bi ih odgovarajući moderatori videli.

Ovo se može postaviti [dodavanjem nekih grupa](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) i zatim specificiranjem odgovarajućih `Moderation Group` ID‑ova u vidžetu za komentare,
[kao što je opisano ovde](/guide-customizations-and-configuration.html#moderation-group-ids).