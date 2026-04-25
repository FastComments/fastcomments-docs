Za večino spletnih mest je najlažji način za dodajanje komentarjev pripeti polje `FastComments` k vašim tipom vsebine. Pojdite na `Structure > Content types > [type] > Manage fields` in dodajte polje.

Vsaka entiteta, ki ima to polje, prejme:

- **Preklopnik stanja**, da lahko uredniki za vsako entiteto posebej vklopijo ali izklopijo komentiranje.
- Neobvezni **prilagojen identifikator**, da lahko uporabite stabilen ID, ki ni vezan na pot entitete v Drupalu.

Glavni blok `FastComments Widget` upošteva to polje in bo preskočil entitete, ki imajo to polje že pripeto. Na ta način lahko združite komentiranje po entitetah z blokom, ne da bi na isti strani videli pripomoček dvakrat.