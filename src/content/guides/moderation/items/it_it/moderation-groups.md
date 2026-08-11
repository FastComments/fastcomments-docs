---
I moderatori possono essere inseriti in gruppi per moderare diverse pagine o categorie di contenuti.

Quando un moderatore appartiene a uno o più gruppi, vedrà solo i commenti di quei gruppi nella pagina Modera Commenti.

Ad esempio, supponiamo di gestire un sito che mostra video per categoria. Potremmo voler avere moderatori diversi per i video di Gatto, Cane e Pappagallo, quindi [aggiungiamo quei gruppi](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; alt='Elenco dei gruppi di moderazione con i gruppi Gatto, Cane e Pappagallo creati per ogni categoria di video'; title='La pagina dei gruppi di moderazione' app-screenshot-end]

Quando aggiungiamo un moderatore, ora abbiamo la possibilità di selezionare uno o più gruppi a cui il moderatore appartiene:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='Modulo Aggiungi Moderatore con il selettore di gruppo usato per assegnare il moderatore a uno o più gruppi'; title='Aggiungere un Moderatore e Selezionare un Gruppo' app-screenshot-end]

Infine, i commenti devono essere associati a uno o più gruppi affinché i moderatori corretti li vedano.

Questo può essere configurato [aggiungendo alcuni gruppi](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) e poi specificando gli ID del `Moderation Group` corrispondenti nel widget dei commenti,
[come indicato qui](/guide-customizations-and-configuration.html#moderation-group-ids).

---