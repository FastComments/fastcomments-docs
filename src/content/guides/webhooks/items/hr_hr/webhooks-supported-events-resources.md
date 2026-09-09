---
FastComments podržava webhooks samo za resurs Comment.

Podržavamo webhooks za stvaranje komentara, uklanjanje i ažuriranje.

Svaki od ovih smatra se zasebnim događajem u našem sustavu i stoga ima različitu semantiku
i strukture za webhook događaje.

Bilo koji broj krajnjih točaka može se pretplatiti na isti događaj: jedan webhook po domeni može se konfigurirati u
dashboardu, a dodatne pretplate mogu se kreirati putem API-ja (pogledajte Managing Webhooks via the API).

---