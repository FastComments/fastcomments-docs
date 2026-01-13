### Broadcast ID-ovi

Vidjet ćete da trebate proslijediti `broadcastId` u nekim API pozivima. Kada primite događaje, dobit ćete ovaj ID natrag, tako da znate zanemariti događaj ako planirate optimistično primijeniti promjene na klijentu
(što ćete vjerojatno poželjeti učiniti jer to pruža najbolje korisničko iskustvo). Pošaljite ovdje UUID. ID bi trebao biti dovoljno jedinstven da se ne pojavi dva puta u sesiji preglednika.