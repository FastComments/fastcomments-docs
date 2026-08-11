---
Moderatorje je mogoče razporediti v skupine, da moderirajo različne strani ali kategorije vsebine.

Ko moderator pripada eni ali več skupinam, bo na strani Moderiraj komentarje videl le komentarje iz teh skupin.

Na primer, recimo, da upravljamo spletno mesto, ki prikazuje videoposnetke po kategorijah. Morda želimo imeti različne moderatorje za videoposnetke mačk, psov in papig, zato [dodajmo te skupine](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; alt='Seznam skupin moderacije z ustvarjenimi skupinami Mačka, Pes in Papiga za vsako kategorijo videoposnetkov'; title='Stran skupin moderacije' app-screenshot-end]

Ko dodamo moderatorja, imamo zdaj možnost izbrati eno ali več skupin, katerih član bo moderator:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='Obrazec za dodajanje moderatorja z izbirnikom skupin, ki se uporablja za dodelitev moderatorja eni ali več skupinam'; title='Dodajanje moderatorja in izbira skupine' app-screenshot-end]

Na koncu je treba komentarje povezati z eno ali več skupinami, da jih vidijo ustrezni moderatorji.

To lahko nastavite z [dodajanjem nekaj skupin](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) in nato z določanjem ustreznih `Moderation Group` ID-jev v pripomočku za komentarje,
[kot je opisano tukaj](/guide-customizations-and-configuration.html#moderation-group-ids).