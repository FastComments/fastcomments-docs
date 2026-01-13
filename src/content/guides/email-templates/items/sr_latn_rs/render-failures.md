Pošto email šabloni podržavaju promenljive i logiku, moguće je napraviti šablone
koji ne uspeju da se renderuju, ili ponekad ne uspeju da se renderuju.

Ovo može biti veoma frustrirajuće za dijagnostikovanje i otklanjanje grešaka, naročito ako je u pitanju povremeni problem, ili
ako se javlja samo kada podaci izgledaju na određeni način.

Da bi pomogao, FastComments Email Templates ima nekoliko funkcija:

1. Ako šablon ne uspe da se prikaže u pregledu, ne može se sačuvati. Prikazaće se poruka o grešci.
2. Greške pri renderovanju šablona se prate i izveštavaju u administratorskom interfejsu.

Druga tačka opisuje greške pri renderovanju koje se događaju u produkciji. Kao, kreirate šablon koji se u pregledu
prikazuje ispravno - ali kasnije iz nekog razloga zakaže. Na primer, ako imamo ovo u našem šablonu:

    <% if (comment.commenterEmail.includes('test') { %>

Ovo može ponekad da zakaže ako imamo omogućeno anonimno komentarisanje, pošto email neće uvek
biti dostupan. Pa kako to saznamo?

Odgovor je da se greške prikazuju na dva mesta. Prvo, sama lista šablona
prikazuje broj grešaka pri renderovanju uz svaki šablon.

Zatim, kada pregledamo šablon možemo videti broj, po grešci, koliko puta je šablon
nije uspeo da se renderuje.

Dugme za reset se nalazi pored svake greške i njenog broja, tako da možemo resetovati brojač
nakon što smo rešili problem.