Uobičajeno je imati po jedan sub-tenant za svako test ili dev okruženje u FastComments. Svaki tenant bi imao svoju konfiguraciju, podatke i API ključeve. Konfiguracija, podaci i korisnici se ne mogu deliti između tenant-a.
Sve je izolovano. Međutim, super administratori roditeljskog tenant-a mogu se predstavljati kao korisnici u podređenim tenant-ima.

Postoje dva pristupa:

- Glavni tenant je za produkciju, a sub-tenanti su za test okruženja.
- Glavni tenant služi samo za naplatu, a svaki sub-tenant je za prod, test i slično.

Prvi pristup je generalno jednostavniji za korisnike da razumeju, ali to može zavisiti od vaše organizacije.

Tenante možete kreirati [ovde](https://eu.fastcomments.com/auth/my-account/tenants) ako imate paket. Takođe je ovo mesto gde bi se super administratori predstavljali kao korisnici. Tenante je moguće kreirati i preko API-ja za prilagođenija/automatizovana rešenja.

Bez obzira na pristup koji odaberete, moraćete da dodate moderatore i korisnike koji žele da vide podatke iz produkcije u "prod" tenant-u. Dakle, na primer, ako želite da idete sa opcijom B i imate roditeljski tenant za naplatu, i imate sub-tenant za "prod", treba da dodate tenant, prebacite se na novi tenant i dodate svoje admin i moderator korisnike za sub-tenant.

Na kraju, da razjasnimo, stranica Moderate Comments će biti prazna za roditeljski tenant ako koristite opciju B.