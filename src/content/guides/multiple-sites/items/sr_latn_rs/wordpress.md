Svaki WordPress sajt treba da ima svoj FastComments nalog ili tenant. WordPress numeriše postove odvojeno na svakom sajtu, tako da bi post 5 na jednom sajtu i post 5 na drugom delili istu nit komentara ako su oba povezana na isti nalog. Iz tog razloga, podešavanje plugina odbija da poveže sajt kada nalog već ima drugi sajt povezan.

[Možete pogledati svoje aktivne integracije ovde.](https://fastcomments.com/auth/my-account/manage-data/integrations)

Da biste upravljali sa više WordPress sajtova, kreirajte [tenant](https://fastcomments.com/auth/my-account/tenants) za svaki, prebacite se na njega, i zatim povežite sajt sa stranice za podešavanje plugina. Tenant‑i su uključeni u planove sa white labeling‑om. Takođe možete kreirati poseban FastComments nalog za svaki sajt, koristeći različitu email adresu za svaki.

Ako je sajt premešten, na primer sa staging‑a na produkciju, deaktivirajte plugin na starom sajtu i uklonite stari sajt iz vaših aktivnih integracija. Zatim povežite novi. Komentari koji su već sinhronizovani sa starog sajta ostaju na nalogu i prikazuju se na svakom postu sa istim ID‑jem na novom sajtu.

Možete koristiti isti FastComments nalog sa jedinstvenom WordPress instalacijom koja obuhvata više domena i ima isti sadržaj na tim domenima.