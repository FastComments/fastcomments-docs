FastComments pruža automatizovan način za migraciju vaših komentara između domena.

Migracija domena jednostavno zahteva `from` i `to` domen.

Ovo **premešta** komentare, ne kopira ih. Ako želite da kopirate komentare, obratite nam se.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='Alat za migraciju domena sa poljima za izvorni i ciljni domen i brojem migriranih komentara'; title='Migracija domena' app-screenshot-end]

Ovo je takođe korisno, na primer, ako deo vaše migracije na FastComments uključuje prelazak sa drugog provajdera, tako da vaši podaci za uvoz komentara
mogu sadržati podatke koje treba migrirati. U tom slučaju, možete pokrenuti uvoz, a zatim migraciju domena.

### Praćenje napretka

Alat za migraciju domena koristi isti sistem obrade poslova FastComments kao i ostali alati za upravljanje podacima.

Može postojati kašnjenje pre nego što vaša migracija počne. To je normalno, jer sistem periodično proverava postoje li novi poslovi za obradu.

Dok se posao izvršava, prikazaće broj pronađenih komentara za migraciju i broj do sada migriranih.

### Migracija domena putem API-ja

Domeni se takođe mogu migrirati pomoću [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post), sa `type` postavljenim na `domain`. Napredak je dostupan putem [`GET /api/v1/batch-jobs/:id`](/guide-api.html#batch-job-get-by-id), a migracija se može poništiti u roku od 90 dana.