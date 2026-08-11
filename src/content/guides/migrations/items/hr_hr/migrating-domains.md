FastComments pruža automatizirani način za migraciju vaših komentara između domena.

Migracija domena jednostavno zahtijeva `from` i `to` domenu.

Ovo **premješta** komentare, ne kopira ih. Ako želite kopirati komentare, obratite nam se.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='Alat za migraciju domena s poljima za from i to domenu i brojem migriranih komentara'; title='Migriranje domena' app-screenshot-end]

Ovo je također korisno, na primjer, ako dio vaše migracije na FastComments uključuje migraciju s drugog pružatelja, pa vaš uvezeni podaci o komentarima mogu sadržavati podatke koji trebaju biti migrirani. U tom slučaju možete pokrenuti uvoz, a zatim migraciju domena.

### Praćenje napretka

Alat za migraciju domena koristi isti FastComments sustav obrade poslova kao i ostali alati za upravljanje podacima.

Možda će doći do kašnjenja prije nego što vaša migracija započne. To je normalno, jer sustav povremeno provjerava nove poslove za obradu.

Dok posao radi, prikazat će broj pronađenih komentara za migraciju i broj dosad migriranih komentara.