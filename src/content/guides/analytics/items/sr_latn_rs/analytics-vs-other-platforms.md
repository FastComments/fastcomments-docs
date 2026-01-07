Možda ćete primetiti da naše Analytics metrike pokazuju malo drugačije brojeve od, recimo, Google Ads © ili sličnih proizvoda.

Za veb stranice sa jednim vidžetom za komentare po stranici, brojevi koje pruža FastComments Analytics su veoma tačni, a ako su netačni biće **niži** od stvarne vrednosti, ali ne više.

Ako imate SPA, možda ćete primetiti da su brojevi FastComments Analytics viši od onih koje prijavljuju vaši marketinški proizvodi. To je zato što marketinški proizvod možda prati samo kada stranica nije učitana, ali ne i svaki put kada korisnik uradi nešto na stranici što bi moglo pokrenuti prikaz nove niti komentara, što bi se računalo kao učitavanje stranice za FastComments Analytics.

#### Tehničke informacije

FastComments Analytics prati svako učitavanje stranice i ne oslanja se na slučajnost kao optimizaciju. Svako učitavanje stranice rezultira ažuriranjem brojača u memoriji u svakoj niti na svakom serveru, koji se zatim povremeno čuva u bazi podataka putem atomske operacije.
