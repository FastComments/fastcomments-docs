Privzeto FastComments vključuje učljiv sistem za zaznavanje neželene pošte.

Ko moderirate komentarje in jih označite kot **Neželeno**, ali označite komentarje, ki so bili samodejno zaznani kot **Neželeno**, kot **Ni neželeno**, se bo sistem za zaznavanje neželene pošte iz teh dejanj učil, da natančneje ugotovi, kaj želite označiti kot neželeno pošto.

Komentarji, označeni kot **Neželeno**, ne bodo samodejno odobreni, zato se ne bodo prikazali, dokler jih izrecno ne označite kot **Ni neželeno**.

Zaznavanje neželene pošte lahko onemogočite na strani Nastavitve moderiranja komentarjev.

### Različni detektorji neželene pošte

FastComments podpira tri načine zaznavanja neželene pošte:

1. Tradicionalni Naïve-Bayes klasifikator, ki se neprestano uči in je deljen med vsemi najemniki FastComments.com.
2. Tradicionalni Naïve-Bayes klasifikator, ki se neprestano uči in je **izoliran** za vašega najemnika.
3. Uporaba ChatGPT 4.

Vsi imajo dostop do deljenega in izoliranega Naïve-Bayes klasifikatorja.

Možnost ChatGPT 4 je izbirna na strani Nastavitve moderiranja komentarjev, če ste na Flex obračunavanju, saj se zaračunava glede na uporabljene tokene.

### Faktor zaupanja

FastComments prilagodi filter neželene pošte za uporabnika glede na to, koliko mu je na določeni strani zaupano.

Na primer, če so skrbniki pripeli veliko njihovih komentarjev, potem so verjetno zelo zaupanja vredni uporabniki. Ali pa, če so bili član spletnega mesta dolgo časa in imajo veliko komentarjev, je lahko njihov faktor zaupanja visok.

### SSO

Komentarji, objavljeni s strani SSO uporabnikov, so lahko obravnavani kot neželena pošta in bodo tako tudi preverjeni. Izjema je, če ima SSO uporabnik enak e-poštni naslov kot uporabnik najemnika, ki ima eno ali več naslednjih dovoljenj:

- Account Owner
- Super Admin
- Comment Moderator Admin

SSO uporabniki s temi dovoljenji ne bodo imeli svojih komentarjev preverjenih za neželeno pošto.

### Ponovljena sporočila

FastComments bo zaznal in preprečil ponovljena sporočila. Prav tako bo zaznal ponovljena sporočila, ki so zelo podobna, da bi pomagal preprečiti neželeno pošto. To ni mogoče onemogočiti, saj preprečuje zlorabo naše platforme. Če imate visok faktor zaupanja, se to upošteva pri preprečevanju ponovljenih sporočil.