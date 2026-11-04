Vsaka WordPress spletna stran potrebuje svoj FastComments račun ali tenant. WordPress številči objave ločeno na vsaki spletni strani, zato bi objava 5 na eni strani in objava 5 na drugi delili isto nit komentarjev, če bi bili oba povezana na isti račun. Zato namestitev vtičnika zavrne povezavo spletnega mesta, če ima račun že povezano drugo spletno mesto.

[Svoje aktivne integracije si lahko ogledate tukaj.](https://fastcomments.com/auth/my-account/manage-data/integrations)

Za upravljanje več WordPress spletnih strani ustvarite [tenant](https://fastcomments.com/auth/my-account/tenants) za vsako od njih, preklopite nanj in nato povežite spletno mesto z nastavitvene strani vtičnika. Tenanti so vključeni v pakete z white labeling. Za vsako spletno mesto lahko ustvarite tudi ločen FastComments račun, pri čemer uporabite drugačen e-poštni naslov za vsakega.

Če se je spletno mesto premaknilo, na primer iz staging v produkcijo, deaktivirajte vtičnik na starem spletnem mestu in odstranite staro spletno mesto iz svojih aktivnih integracij. Nato povežite novo. Komentarji, ki so že bili sinhronizirani s starega spletnega mesta, ostanejo na računu in se prikažejo na vsaki objavi z istim ID-jem na novem spletnem mestu.

Isti FastComments račun lahko uporabite z eno samo WordPress namestitvijo, ki zajema več domen in ima na teh domenah isto vsebino.