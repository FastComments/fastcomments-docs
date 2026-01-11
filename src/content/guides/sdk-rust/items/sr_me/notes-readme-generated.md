### Broadcast ID-ovi

Videćete da treba da proslijedite `broadcastId` u nekim API pozivima. Kada primite događaje, dobićete ovaj ID nazad, pa ćete znati da ignorišete događaj ako planirate optimistično primijeniti promjene na klijentu
(što ćete vjerovatno htjeti uraditi, jer to pruža najbolje iskustvo). Pošaljite ovdje UUID. ID treba da bude dovoljno jedinstven da se ne pojavi dvaput u toku iste sesije pregledača.