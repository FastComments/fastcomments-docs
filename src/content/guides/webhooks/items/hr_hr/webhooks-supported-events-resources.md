---
FastComments podržava webhookove samo za resurs Comment.

Podržavamo webhookove za stvaranje komentara, uklanjanje i ažuriranje.

Svaki od ovih smatra se zasebnim događajem u našem sustavu i stoga ima različitu semantiku
i strukture za webhook događaje.

Bilo koji broj krajnjih točaka može se pretplatiti na isti događaj, putem nadzorne ploče ili kroz API
(pogledajte Upravljanje webhookovima putem API-ja). Svaki webhook se isporučuje neovisno.

---