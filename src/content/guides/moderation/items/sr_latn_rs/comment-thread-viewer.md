Kada moderirate i pregledate niti komentara, poželjno je moći direktno skočiti na nit da biste dobili kontekst tokom moderacije.

To znači da korisnikov tok počinje na stranici za moderisanje komentara, i da bi potom morao ići od pojedinačnog komentara do
stranice koja sadrži taj komentar, čekati da se stranica učita, čekati da se komentari učitaju, i zatim skrolovati do tog komentara.

Međutim, FastComments pruža brži način. Na stranici za moderisanje komentara, pored svakog komentara, u donjem desnom uglu nalazi se dugme "Pogledaj komentar".

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=1&demo=true'; linkUrl='/auth/my-account/moderate-comments'; selector = '.comments .comment-component'; title='A Comment' app-screenshot-end]

Ako ovaj komentar ima odgovore, tekst na dugmetu će umesto toga prikazati broj odgovora, ali klik na njega izvodi istu radnju.

Ovo dugme vodi do **Pregledača niti komentara**.

Pregledač niti komentara je mala, brzo učitavajuća aplikacija koju hostuje FastComments koja renderuje nit komentara za stranicu koja
sadrži komentar, i skroluje do tog komentara.

Ovo omogućava moderatorima da brzo pribave potreban kontekst, bez čekanja da se druga stranica učita.