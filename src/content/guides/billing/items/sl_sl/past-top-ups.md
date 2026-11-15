Tabela `Pretekli polog` na dnu strani s krediti prikazuje vaše najnovejše pologe, od najnovejšega do najstarejšega, z `Datumom`, `Zneskom` in `Statusom` vsakega. Pred vašim prvim pologom se prikaže `Ni še pologov.`

Vsak polog ima enega od treh statusov:

- `Credited`: plačilo je uspelo in znesek je bil dodan vašemu stanju.
- `Pending`: plačilo se je začelo, vendar ni končano, običajno zato, ker potrditve banke ni bilo zaključeno. Pologi v teku se ob naslednjem odpiranju strani s krediti preverijo pri Stripe in preidejo v `Credited` ali `Failed`.
- `Failed`: kartica je bila zavrnjena ali banka ni odobrila plačila. Nič ni bilo bremenjeno in vaše stanje se ni spremenilo.

Tabela beleži pologe, opravljene s te strani. Računi, plačani s kreditom, so prikazani skupaj z računi samimi, pod `View Invoices` na strani [Billing Info](https://fastcomments.com/auth/my-account/billing-info).