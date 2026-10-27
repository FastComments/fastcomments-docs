FastComments pruža automatizovan način da migrirate svoje komentare između domena.

Migracija domena jednostavno zahteva `from` i `to` domen.

Ovo **premesta** komentare, ne kopira ih. Ako želite da kopirate komentare, obratite nam se.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='Alat za migraciju domena sa poljima za izvorni i odredišni domen i brojem migriranih komentara'; title='Migracija domena' app-screenshot-end]

Ovo je takođe korisno, na primer, ako deo vaše migracije na FastComments podrazumeva migraciju sa drugog provajdera, pa podaci uvoza komentara mogu sadržati podatke koji treba da se migriraju. U tom slučaju, možete pokrenuti uvoz, a zatim migraciju domena.

### Monitoring Progress

Alat za migraciju domena koristi isti FastComments sistem za obradu poslova kao i ostali alati za upravljanje podacima.

Može doći do kašnjenja pre nego što migracija počne. Ovo je normalno, jer sistem periodično proverava nove poslove za obradu.

Dok posao radi, prikazaće broj komentara koji su pronađeni za migraciju i broj dosad migriranih.

### Migrating Domains via API

Domeni se takođe mogu migrirati pomoću [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post), postavljanjem `type` na `domain`. Napredak je dostupan putem [`GET /api/v1/batch-jobs/:id`](/guide-api.html#batch-job-get-by-id), a migracija se može poništiti 90 dana.

---