---
I Moderatori possono essere inseriti in gruppi per moderare diverse pagine o categorie di contenuto.

Quando un Moderatore appartiene a uno o più gruppi, vedrà solo i commenti di quei gruppi nella pagina Moderate Comments.

Ad esempio, supponiamo di gestire un sito che mostra video per categoria. Potremmo voler avere moderatori diversi per i video di Gatti, Cani e Pappagalli, quindi [aggiungiamo quei gruppi](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; title='The Moderation Groups Page' app-screenshot-end]

Quando aggiungiamo un moderatore, ora abbiamo l'opzione di selezionare uno o più gruppi a cui il moderatore apparterrà:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; title='Adding A Moderator and Selecting a Group' app-screenshot-end]

Infine, i commenti devono essere collegati a uno o più gruppi in modo che i moderatori corretti li vedano.

Questo può essere configurato aggiungendo alcuni gruppi e poi specificando gli ids corrispondenti di `Moderation Group` nel widget dei commenti,
[come indicato qui](/guide-customizations-and-configuration.html#moderation-group-ids).

---