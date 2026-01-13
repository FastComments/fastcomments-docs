### Broadcast identifikatori

Vidjet ćete da trebate proslijediti `broadcastId` u nekim API pozivima. Kada primite događaje, dobit ćete taj ID natrag, tako da znate zanemariti događaj ako planirate optimistično primijeniti promjene na klijentu
(što ćete vjerojatno htjeti učiniti jer pruža najbolje iskustvo). Ovdje proslijedite UUID. ID bi trebao biti dovoljno jedinstven da se ne pojavi dva puta u jednoj sesiji preglednika.