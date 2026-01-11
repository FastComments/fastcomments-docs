### Broadcast ID-ovi

Videćete da treba da prosledite `broadcastId` u nekim API pozivima. Kada primite događaje, dobićete ovaj ID nazad, tako da znate da zanemarite događaj ako planirate da optimistično primenite promene na klijentu
(što ćete verovatno želeti da uradite, jer pruža najbolje iskustvo). Ovde prosledite `UUID`. ID bi trebalo da bude dovoljno jedinstven da se ne pojavi dva puta u toku jedne sesije pregledača.