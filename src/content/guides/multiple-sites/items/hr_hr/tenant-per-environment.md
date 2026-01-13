Uobičajeno je imati pod-tenant za svako testno ili razvojno okruženje s FastComments. Svaki tenant ima svoju konfiguraciju, podatke i API ključeve. Konfiguracija, podaci i korisnici ne mogu se dijeliti između tenant-a.
Sve je izolirano. Međutim, super administratori roditeljskog tenanta mogu se predstavljati kao korisnici u podređenim tenantima.

Postoje dva pristupa:

- Glavni tenant je za produkciju, a pod-tenanti su za testna okruženja.
- Glavni tenant je jednostavno za naplatu, a svaki pod-tenant je za prod, test i slično.

Prvi pristup je općenito lakši za razumijevanje korisnicima, ali to može ovisiti o vašoj organizaciji.

Tenante možete kreirati [ovdje](https://eu.fastcomments.com/auth/my-account/tenants) ako imate paket. Ovo je također mjesto gdje bi super admini
predstavljali korisnike. Tenante je također moguće kreirati putem API-ja za prilagođenija/automatizirana rješenja.

Bez obzira na odabrani pristup, morat ćete dodati moderatore i korisnike koji žele vidjeti producijske podatke u "prod" tenantu. Dakle, na primjer, ako želite
odabrati opciju B i imati roditeljski tenant za naplatu te pod-tenant za "prod", trebali biste dodati tenant, prebaciti se na novi tenant i dodati svoje
admin i moderator korisnike za pod-tenant. 

Za kraj, radi pojašnjenja, stranica Moderate Comments bit će prazna kod opcije B za roditeljski tenant.