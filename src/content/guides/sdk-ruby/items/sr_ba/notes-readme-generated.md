### Broadcast ID-ovi

Vidjet ćete da treba proslijediti `broadcastId` u nekim API pozivima. Kada primite događaje, dobit ćete ovaj ID nazad, tako da znate da ignorišete događaj ako planirate optimistično primijeniti promjene na klijentu
(što ćete vjerovatno htjeti uraditi, budući da pruža najbolje iskustvo). Ovdje proslijedite UUID. ID bi trebao biti dovoljno jedinstven da se ne pojavi dva puta u jednoj sesiji preglednika.

### SSO (Jedinstveno prijavljivanje)

Za primjere SSO-a, pogledajte dolje.