Som standard vil FastComments automatisk registrere, om dit websted har en mørk baggrund baseret på "afstanden fra sort" i farvecirklen.

Vores produkter gør deres bedste for dette, men der er næsten uendeligt mange farver i farvehjulet, og der kan være scenarier, hvor applikationen vælger at bruge mørk tilstand, når det ikke er passende, og omvendt. Denne dokumentation dækker, hvordan du får mere finkornet kontrol over dette.

#### Tekniske detaljer

Vi registrerer mørk tilstand ved at gennemgå elementerne på siden op fra kommentar-widgetten og lede efter en mørk baggrund, når widgetten indlæses første gang.

For at skifte mørk tilstand efter dette trin skal du kalde widgetten for at opdatere dens konfiguration. Dette er dækket i afsnittet `Manuel konfiguration`.
