[Val Town](https://val.town) izvaja TypeScript na Deno, zato je val pravi strežnik. To ga naredi primerno za FastComments: gradnik je oznaka script na strani, in vse, kar potrebuje skrivnost, kot je varna SSO ali preverjanje webhooka, lahko teče na strežniški strani v istem valu.

Ta vodnik pokriva dodajanje gradnika za komentarje v HTTP val, prikaz števila komentarjev na indeksni strani, prijavo uporabnikov s računom Val Town, ki ga že imajo, in prejemanje webhookov za komentarje.

Za preizkus ne potrebujete računa. Primeri uporabljajo `tenantId: "demo"`, skupni sandbox, in korak 2 opisuje preklop na vašega.