FastComments pruža automatizirani način za migraciju vaših komentara između domena.

Migracija domena jednostavno zahtijeva `from` i `to` domenu.

Ovo **premješta** komentare, ne kopira ih. Ako želite kopirati komentare, obratite nam se.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='Alat za migraciju domena s poljima za od i do domenu i brojem migriranih komentara'; title='Migriranje domena' app-screenshot-end]

Ovo je također korisno, na primjer, ako dio vaše migracije na FastComments uključuje migraciju s drugog pružatelja, pa vaš uvezeni komentar podaci mogu sadržavati podatke koji trebaju biti migrirani. U tom slučaju možete pokrenuti uvoz, a zatim migraciju domena.

### Praćenje napretka

Alat za migraciju domena koristi isti FastComments sustav obrade poslova kao i ostali alati za upravljanje podacima.

Može doći do kašnjenja prije nego što vaša migracija započne. To je normalno, jer sustav povremeno provjerava nove poslove za obradu.

Dok posao traje, prikazat će broj komentara koji su pronađeni za migraciju i broj dosad migriranih komentara.

### Migriranje domena putem API-ja

Domeni se također mogu migrirati pomoću [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post), postavljanjem `type` na `domain`. Napredak je dostupan putem [`GET /api/v1/batch-jobs/:id`](/guide-api.html#batch-job-get-by-id), a migracija se može poništiti 90 dana.