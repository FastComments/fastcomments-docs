Vse spremembe objekta Comment v sistemu sprožijo dogodek, ki se znajde v vrsti.

Prvotni webhook dogodek je običajno poslan v šestih sekundah po nastanku vira dogodka.

To vrsto lahko spremljate v Webhooks admin v primeru, da vaš API preneha delovati.

Če zahteva do vašega API ne uspe, jo bomo ponovno uvrstili v vrsto po urniku.

Ta urnik je `1 Minute * the retry count`. Če klic ne uspe enkrat, bo poskus ponovljen čez
minuto. Če ne uspe dvakrat, bo nato počakal dve minuti, in tako naprej. To je zato, da ne
preobremenimo vašega API, če imate izpade zaradi obremenitve.

Webhooks je mogoče preklicati na [stran z dnevniki](https://fastcomments.com/auth/my-account/manage-data/webhooks/logs).