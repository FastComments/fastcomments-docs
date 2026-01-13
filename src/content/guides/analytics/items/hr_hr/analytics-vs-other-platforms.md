Možda ćete primijetiti da naše Analytics metrike pokazuju malo drugačije brojeve od, recimo, Google Ads © ili sličnih proizvoda.

Za web stranice s jednim widgetom za komentare po stranici, brojevi koje pruža FastComments Analytics su vrlo točni, a ako su netočni bit će **niži** od stvarne vrijednosti, ali ne više.

Ako imate SPA, možda ćete primijetiti da su brojevi FastComments Analytics viši od onih koje prijavljuju vaši marketinški proizvodi. To je zato što marketinški proizvod možda prati samo kada stranica nije učitana, ali ne i svaki put kada korisnik učini nešto na stranici što bi moglo pokrenuti prikaz nove niti komentara, što bi se računalo kao učitavanje stranice za FastComments Analytics.

#### Tehničke informacije

FastComments Analytics prati svako učitavanje stranice i ne oslanja se na slučajnost kao optimizaciju. Svako učitavanje stranice rezultira ažuriranjem brojača u memoriji u svakoj niti na svakom poslužitelju, koji se zatim povremeno sprema u bazu podataka putem atomske operacije.
