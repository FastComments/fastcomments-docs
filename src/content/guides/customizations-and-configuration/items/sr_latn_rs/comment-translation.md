Podrazumevano, FastComments prevodi sopstveni interfejs vidžeta (dugmad, oznake, poruke o greškama) na jezik posetioca, ali se komentari prikazuju tačno onako kako su napisani.

Comment Translation dodaje prevod samih komentara, tako da zajednica koja piše na više jezika može da pročita svaki komentar na svom jeziku.

Postoje dva režima:

- **Automatski**: komentari napisani na jeziku koji se razlikuje od jezika vidžeta prikazuju se prevedeni. Ispod komentara se pojavljuje red „Prevedeno sa španskog" sa linkom **Prikaži original** koji prebacuje između prevoda i neprevedenog teksta.
- **Ručni**: svaki komentar dobija link **Prevedi**. Klikom na njega se taj komentar prevodi na jezik posetioca, sa istim prekidačem **Prikaži original** nakon toga. Komentari koji su već na jeziku posetioca umesto toga to i navode.

Ciljni jezik je jezik vidžeta: podešavanje [Locale / Language](#locale) na istom pravilu prilagođavanja, ako je postavljeno, inače jezik pregledača posetioca. Od posetioca se ništa ne traži — nalog, ekstenzija ili dodatni softver nisu potrebni, i radi isto u ugrađenom vidžetu i na samostalnim stranicama za komentare.

Ovo se uključuje po <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pravilu prilagođavanja</a>, tako da se može primeniti na ceo sajt, jedan domen ili jednu stranicu. Podrazumevano je isključeno.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-translation']; selector = '.comment-translation'; alt='Padajući meni Comment Translation na stranici za prilagođavanje vidžeta sa opcijama Off, Automatic i Manual'; title='Comment Translation' app-screenshot-end]

Prevod je podešavanje na serverskoj strani. Prosleđivanje `commentTranslation` u JavaScript konfiguraciji vidžeta ga ne uključuje; to čini pravilo.

Svaki komentar se prevodi najviše jednom po ciljnom jeziku i rezultat se čuva uz komentar, tako da se komentar koji pročita hiljadu španskih govornika prevodi samo jednom. Ako autor ili moderator izmeni komentar, sačuvani prevod se odbacuje i komentar se ponovo prevodi pri sledećem pregledu. Prevedeni tekst prolazi kroz ista pravila sanitizacije i linkova kao originalni komentar. Komentari koji su predugi ili čiji se jezik ne može otkriti prikazuju se neprevedeni.

Potrošnja se naplaćuje kao LLM tokeni po istoj ceni kao [AI Agents](/guide-ai-agents.html#plans-and-eligibility) i pojavljuje se na istom mestu u detaljima vašeg naplatnog naloga.