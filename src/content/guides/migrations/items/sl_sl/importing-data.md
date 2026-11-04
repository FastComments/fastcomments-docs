Čeprav lahko FastComments podpora pomaga pri migracijah, jih je večino mogoče enostavno izvesti in spremljati brez posredovanja
podporne osebja.

Nativno podpiramo uvoz izvozov naslednjih ponudnikov:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (prek vtičnika ali izvoza XML ali CSV)
- AnyComment (prek WordPress uvoza/izvoza)

Z navigacijo [tukaj](https://fastcomments.com/auth/my-account/manage-data/import) lahko naložimo datoteko, ki vsebuje podatke za migracijo.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; alt='Stran za uvoz FastComments s polji za izbiro ponudnika in nalaganje datoteke izvoza'; title='Obrazec strani za uvoz' app-screenshot-end]

### Spremljanje uvozov

FastComments uporablja sistem obdelave opravil za obdelavo uvozov in izvozov. Ko sistem prevzame vaše opravilo, bo
periodično poročal o statusu opravila v uporabniškem vmesniku za uvoz ali izvoz.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; alt='Stran za uvoz, ki prikazuje tekoče uvozno opravilo in status, ki ga poroča sistem obdelave opravil'; title='Status uvoznega opravila' app-screenshot-end]

Upoštevajte, da je status uvozov in izvozov viden vsem skrbnikom v računu.

Če vaše opravilo ne uspe, se ne bo samodejno ponovno zagnalo. Uvoz bo treba ponovno poskusiti. Če kateri koli uvoz ali izvoz ne uspe,
so naši sistemski skrbniki samodejno obveščeni. Če opazimo težavo, vas bomo kontaktirali, da preverimo, ali lahko pomagamo.

### Ponovno zagon uvoza

Pri nekaterih migracijah je treba uvoz zagnati večkrat. Na primer, pogosto se najprej izvede migracija za testiranje,
nato pa se uvoz ponovno zažene z najnovejšimi podatki, preden se preklopi.

Ponovni uvoz istih vsebin **ne bo ustvaril dvojnikov**.

### Varnost podatkov in potek

Do datotek uvoza ni mogoče dostopati na noben način prek zunanjih zahtev, datoteke uvoza pa so iz našega sistema izbrisane takoj,
ko se uvoz zaključi.