### Broadcast ID-jevi

Videćete da treba da prosledite `broadcastId` u nekim API pozivima. Kada primite događaje, dobićete ovaj ID nazad, tako da znate da ignorišete događaj ako planirate da optimistično primenite promene na klijentu
(što verovatno želite da uradite jer pruža najbolje iskustvo). Ovde prosledite `UUID`. ID bi trebalo da bude dovoljno jedinstven da se ne pojavi dva puta u jednoj sesiji pregledača.

### SSO (Jedinstvena prijava)

Za SSO primere, pogledajte ispod.