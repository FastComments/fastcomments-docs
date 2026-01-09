Pošto email predlošci podržavaju varijable i logiku, moguće je napraviti predloške
koji neće da se renderuju, ili povremeno neće da se renderuju.

Ovo može biti veoma frustrirajuće za dijagnostikovanje i otklanjanje grešaka, posebno ako je problem povremen, ili
ako se javlja samo kada podaci izgledaju na određeni način.

Da bi pomogao, FastComments Email Templates ima nekoliko funkcija:

1. Ako se predložak ne može pregledati, ne može se sačuvati. Prikaže se poruka o grešci.
2. Greške pri renderovanju predložaka se prate i prijavljuju u administratorskom korisničkom interfejsu.

Druga tačka opisuje greške pri renderovanju koje se dešavaju u produkciji. Kao u: kreirate predložak koji se u pregledu
prikazuje ispravno - ali kasnije iz nekog razloga ne uspe. Na primjer, ako u našem predlošku imamo ovo:

    <% if (comment.commenterEmail.includes('test') { %>

Ovo ponekad može da ne uspe ako imamo omogućeno anonimno komentarisanje, jer email adresa neće uvek
biti dostupna. Kako to da saznamo?

Odgovor je da se greške prikazuju na dva mjesta. Prvo, sama lista predložaka
prikazuje broj grešaka pri renderovanju uz svaki predložak.

Zatim, prilikom pregleda predloška možemo vidjeti broj, po grešci, koliko puta se predložak nije uspeo renderovati.

Dugme za resetovanje nalazi se pored svake greške i njenog brojača, kako bismo mogli da resetujemo brojilac
nakon što otklonimo problem.

---