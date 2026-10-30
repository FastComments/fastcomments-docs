---
Privzeto FastComments prevede vmesnik vtičnika (gumba, oznake, sporočila o napakah) v jezik gledalca, vendar so komentarji prikazani natančno takšni, kot so bili napisani.

Prevod komentarjev doda prevajanje samih komentarjev, tako da lahko skupnost, ki piše v več jezikih, prebere vsak komentar v svojem jeziku.

Obstajata dva načina:

- **Samodejno**: komentarji, napisani v jeziku, ki ni jezik vtičnika, so prikazani prevedeni. Pod komentarjem se pojavi vrstica "Prevedeno iz španščine" z povezavo **Prikaži izvirnik**, ki preklaplja med prevodom in neprevedenim besedilom.
- **Ročno**: vsak komentar dobi povezavo **Prevedi**. Klik na to prevede ta komentar v jezik gledalca, z enako možnostjo **Prikaži izvirnik** po tem. Komentarji, ki so že v jeziku gledalca, to sporočijo namesto tega.

Ciljni jezik je jezik vtičnika: nastavitev [Locale / Language](#locale) na isti pravili prilagoditve, če je nastavljena, sicer je jezik brskalnika obiskovalca. Od obiskovalca ni potrebno ničesar, nobenega računa, razširitve ali dodatne programske opreme, in deluje enako v vdelanem vtičniku ter na samostojnih straneh s komentarji.

To je omogočeno po <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pravilu prilagoditve</a>, zato se lahko uporablja za celotno spletno mesto, eno domeno ali posamezno stran. Privzeto je izklopljeno.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-translation']; selector = '.comment-translation'; alt='Spustni meni Prevod komentarjev na strani za prilagoditev vtičnika z možnostmi Izklopljeno, Samodejno in Ročno'; title='Prevod komentarjev' app-screenshot-end]

Prevod je nastavitev na strežniški strani. Posredovanje `commentTranslation` v JavaScript konfiguraciji vtičnika ga ne omogoči; to naredi pravilo.

Vsak komentar je preveden največ enkrat na ciljni jezik, rezultat pa je shranjen skupaj s komentarjem, tako da se komentar, ki ga prebere tisoč španskih govorcev, prevede le enkrat. Če avtor ali moderator komentar ureja, se shranjeni prevod zavrže in komentar se ob naslednjem ogledu ponovno prevede. Prevedeno besedilo gre skozi enako čiščenje in pravila za povezave kot izvirni komentar. Komentarji, ki so predolgi ali katerih jezik ni mogoče zaznati, so prikazani neprevedeni.

Uporaba se obračuna kot LLM žetoni po isti ceni kot [AI Agents](/guide-ai-agents.html#plans-and-eligibility) in se prikaže na istem mestu v vaših podrobnostih obračuna.

---