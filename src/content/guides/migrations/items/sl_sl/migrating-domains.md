FastComments omogoča avtomatiziran način za prenos vaših komentarjev med domenami.

Migracija domen preprosto zahteva domeno `from` in domeno `to`.

To **premika** komentarje, ne kopira jih. Če želite kopirati komentarje, nas kontaktirajte.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='Orodje za migracijo domen z vnosnima poljema from in to ter številom prenesenih komentarjev'; title='Migriranje domen' app-screenshot-end]

To je tudi uporabno, na primer, če del vaše migracije na FastComments vključuje migracijo iz drugega ponudnika, zato lahko vaši podatki za uvoz komentarjev vsebujejo podatke, ki jih je treba migrirati. V tem primeru lahko najprej izvedete uvoz, nato pa migracijo domen.

### Spremljanje napredka

Orodje za migracijo domen uporablja isti FastComments sistem za obdelavo opravil kot ostala orodja za upravljanje podatkov.

Morda bo prišlo do zamude, preden se vaša migracija začne. To je normalno, saj sistem periodično preverja nove naloge za obdelavo.

Ko se naloga izvaja, bo prikazala število najdenih komentarjev za migracijo in število doslej migriranih komentarjev.

### Migriranje domen prek API-ja

Domen lahko tudi migrirate z [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post), pri čemer je `type` nastavljen na `domain`. Napredek je na voljo prek [`GET /api/v1/batch-jobs/:id`](/guide-api.html#batch-job-get-by-id), migracijo pa je mogoče razveljaviti 90 dni.

---