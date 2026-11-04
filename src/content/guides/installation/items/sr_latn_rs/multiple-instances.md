Svaka instanca vidžeta za komentare je izolovana. Zbog toga FastComments po prirodi podržava više od jedne instance po stranici, ili više instanci koje pokazuju na isti thread za razgovor.

U slučaju VanillaJS biblioteke na primer, jednostavno treba da povežete vidžet za komentare sa različitim DOM čvorovima. Ako želite samo da ažurirate trenutni thread na stranici, pogledajte [Switching Comment Threads Without Reloading The Page](guide-customizations-and-configuration.html#switching-comment-threads);

### Sinhronizacija stanja autentifikacije kroz više instanci

Razmotrimo primer prilagođene jednostranične aplikacije (single-page-application) koja je lista često postavljanih pitanja, gde svako pitanje ima svoj thread za komentare.

U ovom slučaju, imamo više instanci FastComments u DOM-u istovremeno.

To je u redu, ali nameće određene izazove za korisničko iskustvo.

Razmotrimo sledeći tok:

1. Korisnik poseti stranicu sa listom pitanja, gde svako pitanje ima svoj vidžet za komentare.
2. Korisnik unese svoje korisničko ime i imejl adresu i postavi pitanje na jednom od thread-ova.
3. Korisnik vidi drugu stavku u FAQ o kojoj ima pitanje.
4. Korisnik želi ponovo da komentariše. Da li mora ponovo da unese imejl adresu i korisničko ime?

U ovom slučaju, FastComments sinhronizuje stanje autentifikacije kroz instance vidžeta umesto vas. U četvrtom koraku, korisnik
će već biti privremeno autentifikovan pošto je uneo svoje korisničko ime i imejl adresu na istoj stranici.