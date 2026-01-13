### Broadcast ID-jevi

Videćete da treba da pošaljete `broadcastId` u nekim API pozivima. Kada primite događaje, dobićete ovaj ID nazad, pa ćete znati da ignorišete događaj ako planirate optimistično primeniti izmene na klijentu
(što ćete verovatno želeti da uradite jer pruža najbolje korisničko iskustvo). Pošaljite ovde UUID. ID treba da bude dovoljno jedinstven da se ne pojavi dva puta u okviru iste sesije pregledača.