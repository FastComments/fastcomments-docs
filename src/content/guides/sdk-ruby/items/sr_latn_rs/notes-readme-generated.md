### Broadcast ID-ovi

Videćete da treba da pošaljete `broadcastId` u nekim API pozivima. Kada primite događaje, dobićete ovaj ID nazad, tako da ćete znati da ignorišete događaj ako planirate da optimistično primenite promene na klijentu
(što ćete verovatno želeti da uradite jer pruža najbolje iskustvo). Prosledite ovde UUID. ID treba da bude dovoljno jedinstven da se ne pojavi dva puta u okviru jedne sesije pregledača.

### SSO (Jedinstvena prijava)

Za primere SSO-a, pogledajte ispod.