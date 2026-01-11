### Broadcast ID-ovi

Vidjet ćete da biste trebali proslijediti `broadcastId` u nekim API pozivima. Kada primite događaje, primit ćete ovaj ID natrag, tako da znate zanemariti događaj ako planirate optimistično primijeniti promjene na klijentu
(što ćete vjerojatno htjeti učiniti jer to pruža najbolje iskustvo). Ovdje proslijedite UUID. ID bi trebao biti dovoljno jedinstven da se ne pojavi dva puta tijekom iste sesije preglednika.

### SSO (Jedinstvena prijava)

Za primjere SSO-a, pogledajte dolje.