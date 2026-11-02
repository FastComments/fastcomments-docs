---
Svaka WordPress stranica treba svoj vlastiti FastComments račun ili tenant. WordPress numerira objave zasebno na svakoj stranici, pa bi objava 5 na jednoj stranici i objava 5 na drugoj dijelile jedan i isti komentar thread ako su obje povezane na isti račun. Iz tog razloga, postavke dodatka odbijaju povezati stranicu kada račun već ima drugu povezanu.

[Možete pregledati svoje aktivne integracije ovdje.](https://fastcomments.com/auth/my-account/manage-data/integrations)

Za upravljanje s nekoliko WordPress stranica, kreirajte [tenant](https://fastcomments.com/auth/my-account/tenants) za svaku, prebacite se na njega i zatim povežite stranicu putem postavki dodatka. Tenant-i su uključeni u planove s white labelingom. Također možete kreirati zaseban FastComments račun za svaku stranicu, koristeći različitu email adresu za svaku.

Ako se stranica premjesti, na primjer s testne (staging) na produkcijsku, deaktivirajte dodatak na staroj stranici i uklonite staru stranicu iz svojih aktivnih integracija. Zatim povežite novu. Komentari koji su već sinkronizirani s starom stranicom ostaju na računu i prikazuju se na bilo kojoj objavi s istim ID-om na novoj stranici.

Možete koristiti isti FastComments račun s jedinstvenom WordPress instalacijom koja obuhvaća više domena i ima isti sadržaj na tim domenama.
---