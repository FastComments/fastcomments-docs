---
Čeprav lahko podpora FastComments pomaga pri migracijah, jih je večino mogoče izvesti in spremljati enostavno brez posredovanja osebja za podporo.

Nativno podpiramo uvoz izvozov od naslednjih ponudnikov:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (prek vtičnika)
- AnyComment (Preko WordPress Import/Export)

Če obiščete [tukaj](https://fastcomments.com/auth/my-account/manage-data/import) lahko naložite datoteko, ki vsebuje podatke za migracijo.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Spremljanje uvozov

FastComments uporablja sistem za obdelavo opravil za obdelavo uvozov in izvozov. Ko sistem prevzame vaše opravilo, bo periodično poročal o stanju opravila v vmesniku za uvoz ali izvoz.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Upoštevajte, da so statusi za uvoze in izvoze vidni vsem skrbnikom v računu.

Če vaše opravilo ne uspe, se ne bo samodejno znova zagnalo. Uvoz bo treba poskusiti znova. Če kateri koli uvoz ali izvoz ne uspe, so naši sistemski skrbniki samodejno obveščeni. Če odkrijemo težavo, vas bomo kontaktirali, da preverimo, ali vam lahko pomagamo.

### Ponovni zagon uvoza

Med nekaterimi migracijami je potrebno uvoz izvesti večkrat. Na primer, pogosto se izvede prvi uvoz za testiranje, nato pa se uvoz ponovno izvede z najnovejšimi podatki pred preklopom.

Ponovni uvoz iste vsebine **ne bo ustvaril podvajanj**.

### Varnost podatkov in iztekanje

Datoteke za uvoz niso na noben način dostopne z zunanjimi zahtevki, uvozne datoteke pa se odstranijo iz našega sistema takoj po zaključku uvoza.

---