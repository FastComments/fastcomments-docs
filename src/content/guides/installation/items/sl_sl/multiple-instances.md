Vsak primerek gradnika za komentarje je izoliran. Zaradi tega FastComments po naravi podpira več primerkov na strani ali več primerkov, ki kažejo na isto klepetalno nit.

V primeru knjižnice VanillaJS na primer gradnik za komentarje preprosto povežete z različnimi vozlišči DOM. Če želite preprosto posodobiti trenutno nit na strani, glejte [Preklop niti komentarjev brez ponovnega nalaganja strani](guide-customizations-and-configuration.html#switching-comment-threads);

### Sinhronizacija stanja overjanja med več primerki

Poglejmo primer lastne enostranske aplikacije, ki je seznam pogostih vprašanj z lastno nitjo komentarjev za vsako.

V tem primeru imamo hkrati več primerkov FastComments v DOM.

To je v redu, vendar prinaša nekaj izzivov za uporabniško izkušnjo.

Razmislimo o tem poteku:

1. Uporabnik obišče stran s seznamom vprašanj, od katerih ima vsako svoj gradnik za komentarje.
2. Uporabnik vnese svoje uporabniško ime in e-poštni naslov ter pusti vprašanje na eni od niti.
3. Vidi drug element pogostih vprašanj, o katerem ima vprašanje.
4. Gre komentirati znova. Ali mora znova vnesti svoj e-poštni naslov in uporabniško ime?

V tem primeru FastComments sinhronizacijo stanja overjanja med primerki gradnikov opravi za vas. V četrtem koraku bo uporabnik že začasno overjen, saj je na isti strani vnesel svoje uporabniško ime in e-poštni naslov.