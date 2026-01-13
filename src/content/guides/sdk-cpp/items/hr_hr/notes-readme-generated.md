### Broadcast identifikatori

Vidjet ćete da trebate proslijediti `broadcastId` u nekim API pozivima. Kad primite događaje, dobit ćete natrag ovaj ID, tako da znate zanemariti taj događaj ako planirate optimistično primijeniti promjene na klijentu
(što ćete vjerojatno htjeti učiniti jer nudi najbolje korisničko iskustvo). Ovdje proslijedite UUID. ID bi trebao biti dovoljno jedinstven da se ne pojavi dva puta u istoj pregledničkoj sesiji.

### SSO (Jedinstvena prijava)

Za primjere SSO-a, pogledajte dolje.