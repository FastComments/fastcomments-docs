---
### Broadcast ID-ovi

Videćete da treba da prosledite `broadcast_id` u nekim API pozivima. Kada primite događaje, dobićete ovaj ID nazad, tako da ćete znati da ignorišete događaj ako planirate optimistično primeniti izmene na klijentu (što ćete verovatno želeti, jer pruža najbolje iskustvo). Prosledite ovde `UUID`. ID bi trebalo da bude dovoljno jedinstven da se ne pojavi dvaput tokom sesije u pregledaču.
---