Svaka instanca widgeta za komentare je izolirana. Zbog toga FastComments inherentno podržava više od jedne instance po stranici, ili više instanci koje upućuju na istu nit razgovora.

U slučaju VanillaJS biblioteke, na primjer, jednostavno trebate povezati widget za komentare s različitim DOM čvorovima. Ako želite jednostavno
ažurirati trenutnu nit na stranici, pogledajte [Switching Comment Threads Without Reloading The Page](guide-customizations-and-configuration.html#switching-comment-threads);

### Sinkronizacija stanja autentifikacije kroz više instanci

Prođimo kroz primjer prilagođene jednostranice koja je popis često postavljanih pitanja s vlastitom nitom komentara.

U ovom slučaju, imamo više instanci FastCommentsa u DOM-u istovremeno.

To je u redu, ali postavlja neke izazove za korisničko iskustvo.

Razmotrite ovaj tok:

1. Korisnik posjeti stranicu s popisom pitanja, svako s vlastitim widgetom za komentare.
2. Korisnik unese svoje korisničko ime i e‑mail i ostavi komentar na jednoj od niti.
3. Vidi drugi FAQ unos o kojem ima pitanje.
4. Ponovo ide komentirati. Mora li ponovno unijeti svoj e‑mail i korisničko ime?

U ovom slučaju, FastComments upravlja sinkronizacijom stanja autentifikacije kroz instance widgeta za vas. U četvrtom koraku, korisnik
će već biti privremeno autentificiran jer je unio svoje korisničko ime i e‑mail na istoj stranici.