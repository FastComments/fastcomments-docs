En stor fordel ved SSR er, at JavaScript ikke er påkrævet. Fordi af dette kan applikationer bygges til at føles "lettere" i mange brugstilfælde.

Derudover kan SSR bruges som en fallback i tilfælde af, at brugeren har JavaScript deaktiveret. På den måde kan kommentartråde stadig gengives, og brugeren
kan stadig svare på kommentarer.

FastComments er allerede godt optimeret, så i de fleste tilfælde er SSR ikke nødvendig. Dog har nogle online fællesskaber brugere, som ikke bruger JavaScript, eller hvor det at deaktivere
det er den foretrukne praksis. Her kan FastComments SSR være nyttigt.

En væsentlig ulempe ved SSR er dog, at man er nødt til at genindlæse siden ved små tilstandsændringer.