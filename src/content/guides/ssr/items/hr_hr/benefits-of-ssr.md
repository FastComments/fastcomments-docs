Glavna prednost SSR-a je što JavaScript nije potreban. Zbog toga se aplikacije mogu izgraditi tako da se u mnogim slučajevima osjećaju "lakšima".

Osim toga, SSR se može koristiti kao zamjena u slučaju da je korisniku onemogućen JavaScript. Na taj način niti komentara se i dalje mogu prikazati, a korisnik
još uvijek može odgovoriti na komentare.

FastComments je već dobro optimiziran, pa u većini slučajeva SSR nije potreban. Međutim, neke online zajednice imaju korisnike koji ne koriste JavaScript, ili je onemogućavanje JavaScript-a preferirana praksa. Tu FastComments SSR može biti koristan.

Međutim, glavni kompromis SSR-a je što je potrebno ponovno učitavanje stranice zbog malih promjena stanja.